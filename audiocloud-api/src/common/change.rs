use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::common::media::{PlayId, RenderId, RequestPlay, RequestRender};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::model::MultiChannelValue;
use crate::common::task::TaskPermissions;
use crate::common::task::{
    ConnectionValues, DynamicInstanceNode, FixedInstanceNode, MediaChannels, MixerChannels, MixerNode, NodeConnection, NodePadId, Task,
    TaskSpec, TimeSegment, TrackMedia, TrackNode, UpdateTaskTrackMedia,
};
use crate::common::time::Timestamped;
use crate::newtypes::{
    AppId, AppMediaObjectId, DynamicInstanceNodeId, FixedInstanceId, FixedInstanceNodeId, MediaObjectId, MixerNodeId, NodeConnectionId,
    ParameterId, SecureKey, TrackMediaId, TrackNodeId,
};
use crate::{json_schema_new_type, ChannelMask};

use self::ModifyTaskError::*;

/// Modify task structure
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ModifyTaskSpec {
    /// Add a track node to the task
    AddTrack {
        /// New track node id
        track_id: TrackNodeId,
        /// Number of channels for the track node
        channels: MediaChannels,
    },
    /// Add media to a track node
    AddTrackMedia {
        /// Track node id
        track_id: TrackNodeId,
        /// Media id within the track node
        media_id: TrackMediaId,
        /// Media object specification
        spec:     TrackMedia,
    },
    /// Update track media on a track node
    UpdateTrackMedia {
        /// Track node id
        track_id: TrackNodeId,
        /// Media id within the track node
        media_id: TrackMediaId,
        /// Media object update
        update:   UpdateTaskTrackMedia,
    },
    /// Delete track media from a track node
    DeleteTrackMedia {
        /// Track node id
        track_id: TrackNodeId,
        /// Media id within the track node
        media_id: TrackMediaId,
    },
    /// Delete a track node from the task (including all media and referencing connections)
    DeleteTrack {
        /// Track node id
        track_id: TrackNodeId,
    },
    /// Add a fixed instance node to the task
    AddFixedInstance {
        /// Fixed instance node id
        fixed_id: FixedInstanceNodeId,
        /// Fixed instance node processing specification
        spec:     FixedInstanceNode,
    },
    /// Add a dynamic instance node to the task
    AddDynamicInstance {
        /// Dynamic instance node id
        dynamic_id: DynamicInstanceNodeId,
        /// Dynamic instance node processing specification
        spec:       DynamicInstanceNode,
    },
    /// Add a mixer node to the task
    AddMixer {
        /// Mixer node id
        mixer_id: MixerNodeId,
        /// Mixer node processing specification
        spec:     MixerNode,
    },
    /// Delete a mixer node from the task (including all referencing connections)
    DeleteMixer {
        /// Moxer node id
        mixer_id: MixerNodeId,
    },
    /// Delete a fixed instance node from the task (including all referencing connections)
    DeleteFixedInstance {
        /// Fixed instance node id
        fixed_id: FixedInstanceNodeId,
    },
    /// Delete dynamic instance node from the task (including all referencing connections)
    DeleteDynamicInstance {
        /// Dynamic instance node id
        dynamic_id: DynamicInstanceNodeId,
    },
    /// Delete a connection from the task (preserving the referenced nodes even if they are now unconnected)
    DeleteConnection {
        /// Connection id
        connection_id: NodeConnectionId,
    },
    /// Add a connection to the task
    AddConnection {
        /// Connection id
        connection_id: NodeConnectionId,
        /// Source node pad
        from:          NodePadId,
        /// Destination node pad
        to:            NodePadId,
        /// Source channel mask
        from_channels: ChannelMask,
        /// Destination channel mask
        to_channels:   ChannelMask,
        /// Volume adjustment on audio passing through the connection
        volume:        f64,
        /// Panning adjustment on the audio passing through the connection
        pan:           f64,
    },
    /// Set connection values
    SetConnectionParameterValues {
        /// Connection id
        connection_id: NodeConnectionId,
        /// Values (parameters) on the connection
        values:        ConnectionValues,
    },
    /// Set fixed instance node values
    SetFixedInstanceParameterValues {
        /// Fixed instance node id
        fixed_id: FixedInstanceNodeId,
        /// Values to set
        values:   HashMap<ParameterId, MultiChannelValue>,
    },
    /// Set dynamic instance node values
    SetDynamicInstanceParameterValues {
        /// Dynamic instance node id
        dynamic_id: DynamicInstanceNodeId,
        /// Values to set
        values:     HashMap<ParameterId, MultiChannelValue>,
    },
}

impl ModifyTaskSpec {
    pub fn get_kind(&self) -> &'static str {
        match self {
            ModifyTaskSpec::AddTrack { .. } => "add_track",
            ModifyTaskSpec::AddTrackMedia { .. } => "add_track_media",
            ModifyTaskSpec::UpdateTrackMedia { .. } => "update_track_media",
            ModifyTaskSpec::DeleteTrackMedia { .. } => "delete_track_media",
            ModifyTaskSpec::DeleteTrack { .. } => "delete_track",
            ModifyTaskSpec::AddFixedInstance { .. } => "add_fixed_instance",
            ModifyTaskSpec::AddDynamicInstance { .. } => "add_dynamic_instance",
            ModifyTaskSpec::AddMixer { .. } => "add_mixer",
            ModifyTaskSpec::DeleteMixer { .. } => "delete_mixer",
            ModifyTaskSpec::AddConnection { .. } => "add_mixer_input",
            ModifyTaskSpec::SetConnectionParameterValues { .. } => "set_input_values",
            ModifyTaskSpec::SetFixedInstanceParameterValues { .. } => "set_fixed_instance_parameter_values",
            ModifyTaskSpec::SetDynamicInstanceParameterValues { .. } => "set_dynamic_instance_parameter_values",
            ModifyTaskSpec::DeleteFixedInstance { .. } => "delete_fixed_instance",
            ModifyTaskSpec::DeleteDynamicInstance { .. } => "delete_dynamic_instance",
            ModifyTaskSpec::DeleteConnection { .. } => "delete_connection",
        }
    }
}

/// Modify a task
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ModifyTask {
    /// Modify task specification
    Spec {
        /// Specification change
        spec: ModifyTaskSpec,
    },
    /// Add or overwrite task security
    SetSecurity {
        /// Secure key to add or overwrite
        key:      SecureKey,
        /// Permissions to set for the secure key
        security: TaskPermissions,
    },
    /// Revoke task security
    RevokeSecurity {
        /// Secure key to revoke
        key: SecureKey,
    },
}

/// A desired state for the task play state
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesiredTaskPlayState {
    /// Play, with sample rate conversion
    Play(RequestPlay),

    /// Rendering is always a F32 WAV at full sample rate, so nothing else needs to happen here
    Render(RequestRender),

    /// Stopped
    Stopped,
}

impl DesiredTaskPlayState {
    pub fn is_stop(&self) -> bool {
        self == &Self::Stopped
    }

    pub fn is_render(&self) -> bool {
        matches!(self, Self::Render(_))
    }

    pub fn is_rendering_of(&self, render: &RequestRender) -> bool {
        matches!(self, DesiredTaskPlayState::Render(desired_render) if desired_render == render)
    }

    pub fn is_playing_of(&self, play: &RequestPlay) -> bool {
        matches!(self, DesiredTaskPlayState::Play(desired_play) if desired_play == play)
    }
}

/// Update task play configuration
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UpdateTaskPlay {
    /// Play identifier
    pub play_id:  PlayId,
    /// If not null, change the mixer node monitored during playback
    pub mixer_id: Option<MixerNodeId>,
    /// If not null, change the time segment within the task timeline
    pub segment:  Option<TimeSegment>,
    /// if not null, seek to a specified location within the task timeline
    pub start_at: Option<f64>,
    /// If not null, overwrite if the task playback is looping or not
    pub looping:  Option<bool>,
}

pub struct SuccessfulRenderNotification {
    pub render_id: RenderId,
    pub object_id: MediaObjectId,
    pub context:   String,
}

pub type RenderNotification = Result<SuccessfulRenderNotification, String>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskPlayState {
    PreparingToPlay(RequestPlay),
    PreparingToRender(RequestRender),
    Playing(RequestPlay),
    Rendering(RequestRender),
    StoppingPlay(PlayId),
    StoppingRender(RenderId),
    Stopped,
}

impl TaskPlayState {
    pub fn is_playing(&self, play_id: PlayId) -> bool {
        matches!(self, Self::Playing(playing) if playing.play_id == play_id)
    }

    pub fn is_rendering(&self, render_id: RenderId) -> bool {
        matches!(self, Self::Rendering(rendering) if rendering.render_id == render_id)
    }

    pub fn is_stopped(&self) -> bool {
        matches!(self, Self::Stopped)
    }

    pub fn satisfies(&self, desired: &DesiredTaskPlayState) -> bool {
        match (self, desired) {
            (Self::Playing(playing), DesiredTaskPlayState::Play(desired_playing)) => playing == desired_playing,
            (Self::Rendering(rendering), DesiredTaskPlayState::Render(desired_rendering)) => rendering == desired_rendering,
            (Self::Stopped, DesiredTaskPlayState::Stopped) => true,
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SessionState {
    pub play_state:         Timestamped<TaskPlayState>,
    pub desired_play_state: Timestamped<DesiredTaskPlayState>,
}

impl Default for SessionState {
    fn default() -> Self {
        Self { play_state:         Timestamped::new(TaskPlayState::Stopped),
               desired_play_state: Timestamped::new(DesiredTaskPlayState::Stopped), }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Error, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ModifyTaskError {
    #[error("Track {node_id} already exists")]
    TrackExists { node_id: TrackNodeId },
    #[error("Fixed instance node {node_id} already exists")]
    FixedInstanceExists { node_id: FixedInstanceNodeId },
    #[error("Dynamic instance node {node_id} already exists")]
    DynamicInstanceExists { node_id: DynamicInstanceNodeId },
    #[error("Mixer node {node_id} already exists")]
    MixerExists { node_id: MixerNodeId },

    #[error("Track {node_id} does not exist")]
    TrackDoesNotExist { node_id: TrackNodeId },
    #[error("Fixed instance {node_id} does not exist")]
    FixedInstanceDoesNotExist { node_id: FixedInstanceNodeId },
    #[error("Dynamic instance {node_id} does not exist")]
    DynamicInstanceDoesNotExist { node_id: DynamicInstanceNodeId },
    #[error("Mixer {node_id} does not exist")]
    MixerDoesNotExist { node_id: MixerNodeId },
    #[error("Connection {connection_id} does not exist")]
    ConnectionDoesNotExist { connection_id: NodeConnectionId },
    #[error("Connection {connection_id} already exist")]
    ConnectionExists { connection_id: NodeConnectionId },
    #[error("Connection {connection_id} already exist: {message}")]
    ConnectionMalformed {
        connection_id: NodeConnectionId,
        message:       String,
    },

    #[error("Media {media_id} on track node {node_id} already exists")]
    MediaExists { node_id: TrackNodeId, media_id: TrackMediaId },
    #[error("Media {media_id} on track node {node_id} does not exist")]
    MediaDoesNotExist { node_id: TrackNodeId, media_id: TrackMediaId },

    #[error("Refusing to add connection - cycle detected")]
    CycleDetected,
}

impl Task {
    pub fn get_media_object_ids<'a>(&'a self) -> impl Iterator<Item = &'a MediaObjectId> + 'a {
        self.spec
            .tracks
            .values()
            .flat_map(|track| track.media.values().map(|media| &media.object_id))
    }

    pub fn generate_changes(&self, other: &Task) -> Vec<ModifyTask> {
        let mut rv = vec![];
        security_changes(&mut rv, &self.security, &other.security);

        rv
    }

    pub fn apply_change(&mut self, modify: ModifyTask) -> Result<(), ModifyTaskError> {
        match modify {
            ModifyTask::Spec { spec } => {
                self.spec.modify(spec)?;
            }
            ModifyTask::SetSecurity { key, security } => {
                self.set_security(key, security)?;
            }
            ModifyTask::RevokeSecurity { key } => {
                self.revoke_security(key)?;
            }
        }

        Ok(())
    }

    pub fn set_security(&mut self, key: SecureKey, security: TaskPermissions) -> Result<(), ModifyTaskError> {
        self.security.insert(key, security);
        Ok(())
    }

    pub fn revoke_security(&mut self, key: SecureKey) -> Result<(), ModifyTaskError> {
        self.security.remove(&key);
        Ok(())
    }
}

impl TaskSpec {
    pub fn get_fixed_instance_ids<'a>(&'a self) -> impl Iterator<Item = &'a FixedInstanceId> + 'a {
        self.fixed.values().map(|fixed| &fixed.instance_id)
    }

    pub fn get_media_object_ids(&self, app_id: &AppId) -> HashSet<AppMediaObjectId> {
        self.tracks
            .values()
            .flat_map(|track| track.media.values().map(|media| media.object_id.clone().for_app(app_id.clone())))
            .collect()
    }

    pub fn modify(&mut self, modify: ModifyTaskSpec) -> Result<(), ModifyTaskError> {
        match modify {
            ModifyTaskSpec::AddFixedInstance { fixed_id: mixer_id,
                                               spec: process, } => self.add_fixed_instance(mixer_id, process),
            ModifyTaskSpec::AddDynamicInstance { dynamic_id: mixer_id,
                                                 spec: process, } => self.add_dynamic_instance(mixer_id, process),
            ModifyTaskSpec::AddMixer { mixer_id, spec: channels } => self.add_mixer(mixer_id, channels),
            ModifyTaskSpec::DeleteMixer { mixer_id } => self.delete_mixer(mixer_id),
            ModifyTaskSpec::SetFixedInstanceParameterValues { fixed_id: id, values } => {
                self.set_fixed_instance_parameter_values(id, values)
            }

            ModifyTaskSpec::SetDynamicInstanceParameterValues { dynamic_id: id, values } => {
                self.set_dynamic_instance_parameter_values(id, values)
            }
            ModifyTaskSpec::AddTrack { track_id, channels } => self.add_track(track_id, channels),
            ModifyTaskSpec::DeleteTrackMedia { track_id, media_id } => self.delete_track_media(track_id, media_id),
            ModifyTaskSpec::DeleteTrack { track_id } => self.delete_track(track_id),
            ModifyTaskSpec::SetConnectionParameterValues { connection_id, values } => {
                self.set_connection_parameter_values(connection_id, values)
            }
            ModifyTaskSpec::AddTrackMedia { track_id, media_id, spec } => self.add_track_media(track_id, media_id, spec),
            ModifyTaskSpec::UpdateTrackMedia { track_id,
                                               media_id,
                                               update, } => self.update_track_media(track_id, media_id, update),
            ModifyTaskSpec::DeleteFixedInstance { fixed_id } => self.delete_fixed_instance(fixed_id),
            ModifyTaskSpec::DeleteDynamicInstance { dynamic_id } => self.delete_dynamic_instance(dynamic_id),
            ModifyTaskSpec::DeleteConnection { connection_id } => self.delete_connection(connection_id),
            ModifyTaskSpec::AddConnection { connection_id,
                                            from,
                                            to,
                                            from_channels,
                                            to_channels,
                                            volume,
                                            pan, } => self.add_connection(connection_id, from, to, from_channels, to_channels, volume, pan),
        }
    }

    pub fn add_fixed_instance(&mut self, fixed_id: FixedInstanceNodeId, instance: FixedInstanceNode) -> Result<(), ModifyTaskError> {
        if self.fixed.contains_key(&fixed_id) {
            return Err(FixedInstanceExists { node_id: fixed_id });
        }

        self.fixed.insert(fixed_id, instance);

        Ok(())
    }

    pub fn add_dynamic_instance(&mut self, dynamic_id: DynamicInstanceNodeId, dynamic: DynamicInstanceNode) -> Result<(), ModifyTaskError> {
        if self.dynamic.contains_key(&dynamic_id) {
            return Err(DynamicInstanceExists { node_id: dynamic_id });
        }

        self.dynamic.insert(dynamic_id, dynamic);

        Ok(())
    }

    pub fn add_mixer(&mut self, mixer_id: MixerNodeId, mixer: MixerNode) -> Result<(), ModifyTaskError> {
        if self.mixers.contains_key(&mixer_id) {
            return Err(MixerExists { node_id: mixer_id });
        }

        self.mixers.insert(mixer_id, mixer);

        Ok(())
    }

    pub fn delete_mixer(&mut self, mixer_id: MixerNodeId) -> Result<(), ModifyTaskError> {
        if !self.mixers.contains_key(&mixer_id) {
            return Err(MixerDoesNotExist { node_id: mixer_id });
        }

        self.mixers.remove(&mixer_id);

        Ok(())
    }

    pub fn is_connected(&self, from: &NodePadId, to: &NodePadId) -> bool {
        self.connections
            .iter()
            .any(|(_, connection)| &connection.from == from && &connection.to == to)
    }

    pub fn set_connection_parameter_values(&mut self,
                                           connection_id: NodeConnectionId,
                                           values: ConnectionValues)
                                           -> Result<(), ModifyTaskError> {
        let connection = self.connections
                             .get_mut(&connection_id)
                             .ok_or(ConnectionDoesNotExist { connection_id })?;
        if let Some(volume) = values.volume {
            connection.volume = volume;
        }
        if let Some(pan) = values.pan {
            connection.pan = pan;
        }

        Ok(())
    }

    pub fn set_fixed_instance_parameter_values(&mut self,
                                               node_id: FixedInstanceNodeId,
                                               parameters: HashMap<ParameterId, MultiChannelValue>)
                                               -> Result<(), ModifyTaskError> {
        let fixed = self.fixed.get_mut(&node_id).ok_or(FixedInstanceDoesNotExist { node_id })?;
        // fixed.parameters.extend(parameters.into_iter());
        Ok(())
    }

    pub fn set_dynamic_instance_parameter_values(&mut self,
                                                 node_id: DynamicInstanceNodeId,
                                                 parameters: HashMap<ParameterId, MultiChannelValue>)
                                                 -> Result<(), ModifyTaskError> {
        let dynamic = self.dynamic.get_mut(&node_id).ok_or(DynamicInstanceDoesNotExist { node_id })?;
        // dynamic.parameters.extend(parameters.into_iter());
        Ok(())
    }

    pub fn delete_connections_referencing(&mut self, flow_id: NodePadId) {
        self.connections.retain(|_, value| &value.from != &flow_id && &value.to != &flow_id);
    }

    pub fn add_track(&mut self, track_id: TrackNodeId, channels: MediaChannels) -> Result<(), ModifyTaskError> {
        if self.tracks.contains_key(&track_id) {
            return Err(TrackExists { node_id: track_id });
        }

        self.tracks.insert(track_id,
                           TrackNode { channels,
                                       media: Default::default() });

        Ok(())
    }

    pub fn add_track_media(&mut self, track_id: TrackNodeId, media_id: TrackMediaId, spec: TrackMedia) -> Result<(), ModifyTaskError> {
        let track = self.tracks
                        .get_mut(&track_id)
                        .ok_or(TrackDoesNotExist { node_id: track_id.clone() })?;

        if track.media.contains_key(&media_id) {
            return Err(MediaDoesNotExist { node_id: track_id.clone(),
                                           media_id });
        }

        track.media.insert(media_id, spec);

        Ok(())
    }

    pub fn delete_track_media(&mut self, track_id: TrackNodeId, media_id: TrackMediaId) -> Result<(), ModifyTaskError> {
        let track = self.tracks
                        .get_mut(&track_id)
                        .ok_or(TrackDoesNotExist { node_id: track_id.clone() })?;
        if track.media.remove(&media_id).is_none() {
            Err(MediaDoesNotExist { node_id: track_id.clone(),
                                    media_id })
        } else {
            Ok(())
        }
    }

    pub fn delete_track(&mut self, node_id: TrackNodeId) -> Result<(), ModifyTaskError> {
        if self.tracks.remove(&node_id).is_some() {
            self.delete_connections_referencing(NodePadId::TrackOutput(node_id));

            Ok(())
        } else {
            Err(TrackDoesNotExist { node_id })
        }
    }

    pub fn delete_fixed_instance(&mut self, node_id: FixedInstanceNodeId) -> Result<(), ModifyTaskError> {
        if self.fixed.remove(&node_id).is_some() {
            self.delete_connections_referencing(NodePadId::FixedInstanceOutput(node_id.clone()));
            self.delete_connections_referencing(NodePadId::FixedInstanceInput(node_id.clone()));

            Ok(())
        } else {
            Err(FixedInstanceDoesNotExist { node_id })
        }
    }

    pub fn delete_dynamic_instance(&mut self, node_id: DynamicInstanceNodeId) -> Result<(), ModifyTaskError> {
        if self.dynamic.remove(&node_id).is_some() {
            self.delete_connections_referencing(NodePadId::DynamicInstanceOutput(node_id.clone()));
            self.delete_connections_referencing(NodePadId::DynamicInstanceInput(node_id.clone()));

            Ok(())
        } else {
            Err(DynamicInstanceDoesNotExist { node_id })
        }
    }

    pub fn delete_connection(&mut self, connection_id: NodeConnectionId) -> Result<(), ModifyTaskError> {
        if self.connections.remove(&connection_id).is_some() {
            Ok(())
        } else {
            Err(ConnectionDoesNotExist { connection_id })
        }
    }

    pub fn add_connection(&mut self,
                          connection_id: NodeConnectionId,
                          from: NodePadId,
                          to: NodePadId,
                          from_channels: ChannelMask,
                          to_channels: ChannelMask,
                          volume: f64,
                          pan: f64)
                          -> Result<(), ModifyTaskError> {
        if self.connections.contains_key(&connection_id) {
            return Err(ConnectionExists { connection_id });
        }

        if !from.is_output() {
            return Err(ConnectionMalformed { connection_id,
                                             message: format!("{from} is not an output") });
        }

        if !to.is_input() {
            return Err(ConnectionMalformed { connection_id,
                                             message: format!("{to} is not an input") });
        }

        self.connections.insert(connection_id,
                                NodeConnection { from,
                                                 to,
                                                 from_channels,
                                                 to_channels,
                                                 volume,
                                                 pan });
        Ok(())
    }

    pub fn update_track_media(&mut self,
                              track_id: TrackNodeId,
                              media_id: TrackMediaId,
                              update: UpdateTaskTrackMedia)
                              -> Result<(), ModifyTaskError> {
        let track = self.tracks
                        .get_mut(&track_id)
                        .ok_or(TrackDoesNotExist { node_id: track_id.clone() })?;
        let media = track.media.get_mut(&media_id).ok_or(MediaDoesNotExist { node_id: track_id.clone(),
                                                                              media_id })?;

        media.update(update);

        Ok(())
    }
}

fn security_changes(rv: &mut Vec<ModifyTask>, existing: &HashMap<SecureKey, TaskPermissions>, new: &HashMap<SecureKey, TaskPermissions>) {
    let changes = hashmap_changes(existing, new);
    for (key, security) in changes.changed.into_iter().chain(changes.added.into_iter()) {
        rv.push(ModifyTask::SetSecurity { key, security })
    }
    for key in changes.removed {
        rv.push(ModifyTask::RevokeSecurity { key });
    }
}

fn hashmap_changes<K: Hash + Eq + Clone, T: Clone + PartialEq>(existing: &HashMap<K, T>, new: &HashMap<K, T>) -> HashMapChanges<K, T> {
    let mut changes = HashMapChanges::default();
    let key_set = existing.keys().chain(new.keys()).collect::<HashSet<_>>();
    for key in key_set {
        match (existing.get(key), new.get(key)) {
            (Some(_), None) => {
                changes.removed.insert(key.clone());
            }
            (None, Some(value)) => {
                changes.added.insert(key.clone(), value.clone());
            }
            (Some(existing), Some(new)) if existing != new => {
                changes.changed.insert(key.clone(), new.clone());
            }
            _ => {}
        }
    }

    changes
}

#[derive(Serialize, Deserialize)]
struct HashMapChanges<K: Hash + Eq, T> {
    added:   HashMap<K, T>,
    changed: HashMap<K, T>,
    removed: HashSet<K>,
}

impl<K: Hash + Eq, T> Default for HashMapChanges<K, T> {
    fn default() -> Self {
        Self { added:   HashMap::new(),
               changed: HashMap::new(),
               removed: HashSet::new(), }
    }
}

json_schema_new_type!(NodeConnectionId, PlayId, RenderId);