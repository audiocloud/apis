use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::common::media::{PlayId, RenderId, RequestPlay, RequestRender};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::model::MultiChannelValue;
use crate::common::task::TaskSecurity;
use crate::common::task::{
    ConnectionValues, DynamicInstanceNode, FixedInstanceNode, MediaChannels, MixerChannels, MixerNode, NodeConnection, NodePadId, Task,
    TaskSpec, TimeSegment, TrackMedia, TrackNode, UpdateTaskTrackMedia,
};
use crate::common::time::Timestamped;
use crate::json_schema_new_type;
use crate::newtypes::{
    AppId, AppMediaObjectId, DynamicInstanceNodeId, FixedInstanceId, FixedInstanceNodeId, MediaObjectId, MixerNodeId, NodeConnectionId,
    ParameterId, SecureKey, TrackMediaId, TrackNodeId,
};

use self::ModifyTaskError::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ModifyTaskSpec {
    AddTrack {
        track_id: TrackNodeId,
        channels: MediaChannels,
    },
    AddTrackMedia {
        track_id: TrackNodeId,
        media_id: TrackMediaId,
        spec:     TrackMedia,
    },
    UpdateTrackMedia {
        track_id: TrackNodeId,
        media_id: TrackMediaId,
        update:   UpdateTaskTrackMedia,
    },
    DeleteTrackMedia {
        track_id: TrackNodeId,
        media_id: TrackMediaId,
    },
    DeleteTrack {
        track_id: TrackNodeId,
    },
    AddFixedInstance {
        fixed_id: FixedInstanceNodeId,
        process:  FixedInstanceNode,
    },
    AddDynamicInstance {
        dynamic_id: DynamicInstanceNodeId,
        process:    DynamicInstanceNode,
    },
    AddMixer {
        mixer_id: MixerNodeId,
        mixer:    MixerNode,
    },
    DeleteMixer {
        mixer_id: MixerNodeId,
    },
    DeleteFixedInstance {
        fixed_id: FixedInstanceNodeId,
    },
    DeleteDynamicInstance {
        dynamic_id: DynamicInstanceNodeId,
    },
    DeleteConnection {
        connection_id: NodeConnectionId,
    },
    AddConnection {
        connection_id: NodeConnectionId,
        from:          NodePadId,
        to:            NodePadId,
        from_channels: MixerChannels,
        to_channels:   MixerChannels,
        volume:        f64,
        pan:           f64,
    },
    SetConnectionParameterValues {
        connection_id: NodeConnectionId,
        values:        ConnectionValues,
    },
    SetFixedInstanceParameterValues {
        fixed_id: FixedInstanceNodeId,
        values:   HashMap<ParameterId, MultiChannelValue>,
    },
    SetDynamicInstanceParameterValues {
        dynamic_id: DynamicInstanceNodeId,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ModifyTask {
    Spec(ModifyTaskSpec),
    SetSecurity { key: SecureKey, security: TaskSecurity },
    RevokeSecurity { key: SecureKey },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesiredTaskPlayState {
    /// Play, with sample rate conversion
    Play(RequestPlay),

    /// Rendering is always a F32 WAV at full sample rate, so nothing else needs to happen here
    Render(RequestRender),
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UpdateTaskPlay {
    pub play_id:  PlayId,
    pub mixer_id: Option<MixerNodeId>,
    pub segment:  Option<TimeSegment>,
    pub start_at: Option<f64>,
    pub looping:  bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
#[serde(rename_all = "snake_case")]
pub enum ModifyTaskError {
    #[error("Track {0} already exists")]
    TrackExists(TrackNodeId),
    #[error("Fixed instance {0} already exists")]
    FixedInstanceExists(FixedInstanceNodeId),
    #[error("Dynamic instance {0} already exists")]
    DynamicInstanceExists(DynamicInstanceNodeId),
    #[error("Mixer {0} already exists")]
    MixerExists(MixerNodeId),

    #[error("Track {0} does not exist")]
    TrackDoesNotExist(TrackNodeId),
    #[error("Fixed instance {0} does not exist")]
    FixedInstanceDoesNotExist(FixedInstanceNodeId),
    #[error("Dynamic instance {0} does not exist")]
    DynamicInstanceDoesNotExist(DynamicInstanceNodeId),
    #[error("Mixer {0} does not exist")]
    MixerDoesNotExist(MixerNodeId),
    #[error("Connection {0} does not exist")]
    ConnectionDoesNotExist(NodeConnectionId),
    #[error("Connection {0} already exist")]
    ConnectionExists(NodeConnectionId),
    #[error("Connection {0} already exist: {1}")]
    ConnectionMalformed(NodeConnectionId, String),

    #[error("Media {1} on track {0} already exists")]
    MediaExists(TrackNodeId, TrackMediaId),
    #[error("Media {1} on track {0} does not exist")]
    MediaDoesNotExist(TrackNodeId, TrackMediaId),

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
            ModifyTask::Spec(spec_change) => {
                self.spec.modify(spec_change)?;
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

    pub fn set_security(&mut self, key: SecureKey, security: TaskSecurity) -> Result<(), ModifyTaskError> {
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
                                               process, } => self.add_fixed_instance(mixer_id, process),
            ModifyTaskSpec::AddDynamicInstance { dynamic_id: mixer_id,
                                                 process, } => self.add_dynamic_instance(mixer_id, process),
            ModifyTaskSpec::AddMixer { mixer_id, mixer: channels } => self.add_mixer(mixer_id, channels),
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
            return Err(FixedInstanceExists(fixed_id));
        }

        self.fixed.insert(fixed_id, instance);

        Ok(())
    }

    pub fn add_dynamic_instance(&mut self, dynamic_id: DynamicInstanceNodeId, dynamic: DynamicInstanceNode) -> Result<(), ModifyTaskError> {
        if self.dynamic.contains_key(&dynamic_id) {
            return Err(DynamicInstanceExists(dynamic_id));
        }

        self.dynamic.insert(dynamic_id, dynamic);

        Ok(())
    }

    pub fn add_mixer(&mut self, mixer_id: MixerNodeId, mixer: MixerNode) -> Result<(), ModifyTaskError> {
        if self.mixers.contains_key(&mixer_id) {
            return Err(MixerExists(mixer_id));
        }

        self.mixers.insert(mixer_id, mixer);

        Ok(())
    }

    pub fn delete_mixer(&mut self, mixer_id: MixerNodeId) -> Result<(), ModifyTaskError> {
        if !self.mixers.contains_key(&mixer_id) {
            return Err(MixerDoesNotExist(mixer_id));
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
                             .ok_or(ConnectionDoesNotExist(connection_id))?;
        if let Some(volume) = values.volume {
            connection.volume = volume;
        }
        if let Some(pan) = values.pan {
            connection.pan = pan;
        }

        Ok(())
    }

    pub fn set_fixed_instance_parameter_values(&mut self,
                                               fixed_id: FixedInstanceNodeId,
                                               parameters: HashMap<ParameterId, MultiChannelValue>)
                                               -> Result<(), ModifyTaskError> {
        let fixed = self.fixed.get_mut(&fixed_id).ok_or(FixedInstanceDoesNotExist(fixed_id))?;
        fixed.parameters.extend(parameters.into_iter());
        Ok(())
    }

    pub fn set_dynamic_instance_parameter_values(&mut self,
                                                 dynamic_id: DynamicInstanceNodeId,
                                                 parameters: HashMap<ParameterId, MultiChannelValue>)
                                                 -> Result<(), ModifyTaskError> {
        let dynamic = self.dynamic.get_mut(&dynamic_id).ok_or(DynamicInstanceDoesNotExist(dynamic_id))?;
        dynamic.parameters.extend(parameters.into_iter());
        Ok(())
    }

    pub fn delete_connections_referencing(&mut self, flow_id: NodePadId) {
        self.connections.retain(|_, value| &value.from != &flow_id && &value.to != &flow_id);
    }

    pub fn add_track(&mut self, track_id: TrackNodeId, channels: MediaChannels) -> Result<(), ModifyTaskError> {
        if self.tracks.contains_key(&track_id) {
            return Err(TrackExists(track_id));
        }

        self.tracks.insert(track_id,
                           TrackNode { channels,
                                       media: Default::default() });

        Ok(())
    }

    pub fn add_track_media(&mut self, track_id: TrackNodeId, media_id: TrackMediaId, spec: TrackMedia) -> Result<(), ModifyTaskError> {
        let track = self.tracks.get_mut(&track_id).ok_or(TrackDoesNotExist(track_id.clone()))?;

        if track.media.contains_key(&media_id) {
            return Err(MediaDoesNotExist(track_id.clone(), media_id));
        }

        track.media.insert(media_id, spec);

        Ok(())
    }

    pub fn delete_track_media(&mut self, track_id: TrackNodeId, media_id: TrackMediaId) -> Result<(), ModifyTaskError> {
        let track = self.tracks.get_mut(&track_id).ok_or(TrackDoesNotExist(track_id.clone()))?;
        if track.media.remove(&media_id).is_none() {
            Err(MediaDoesNotExist(track_id.clone(), media_id))
        } else {
            Ok(())
        }
    }

    pub fn delete_track(&mut self, track_id: TrackNodeId) -> Result<(), ModifyTaskError> {
        if self.tracks.remove(&track_id).is_some() {
            self.delete_connections_referencing(NodePadId::TrackOutput(track_id));

            Ok(())
        } else {
            Err(TrackDoesNotExist(track_id))
        }
    }

    pub fn delete_fixed_instance(&mut self, fixed_id: FixedInstanceNodeId) -> Result<(), ModifyTaskError> {
        if self.fixed.remove(&fixed_id).is_some() {
            self.delete_connections_referencing(NodePadId::FixedInstanceOutput(fixed_id.clone()));
            self.delete_connections_referencing(NodePadId::FixedInstanceInput(fixed_id.clone()));

            Ok(())
        } else {
            Err(FixedInstanceDoesNotExist(fixed_id))
        }
    }

    pub fn delete_dynamic_instance(&mut self, dynamic_id: DynamicInstanceNodeId) -> Result<(), ModifyTaskError> {
        if self.dynamic.remove(&dynamic_id).is_some() {
            self.delete_connections_referencing(NodePadId::DynamicInstanceOutput(dynamic_id.clone()));
            self.delete_connections_referencing(NodePadId::DynamicInstanceInput(dynamic_id.clone()));

            Ok(())
        } else {
            Err(DynamicInstanceDoesNotExist(dynamic_id))
        }
    }

    pub fn delete_connection(&mut self, connection_id: NodeConnectionId) -> Result<(), ModifyTaskError> {
        if self.connections.remove(&connection_id).is_some() {
            Ok(())
        } else {
            Err(ConnectionDoesNotExist(connection_id))
        }
    }

    pub fn add_connection(&mut self,
                          connection_id: NodeConnectionId,
                          from: NodePadId,
                          to: NodePadId,
                          from_channels: MixerChannels,
                          to_channels: MixerChannels,
                          volume: f64,
                          pan: f64)
                          -> Result<(), ModifyTaskError> {
        if self.connections.contains_key(&connection_id) {
            return Err(ConnectionExists(connection_id));
        }

        if !from.is_output() {
            return Err(ConnectionMalformed(connection_id, format!("{from} is not an output")));
        }

        if !to.is_input() {
            return Err(ConnectionMalformed(connection_id, format!("{to} is not an input")));
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
        let track = self.tracks.get_mut(&track_id).ok_or(TrackDoesNotExist(track_id.clone()))?;
        let media = track.media
                         .get_mut(&media_id)
                         .ok_or(MediaDoesNotExist(track_id.clone(), media_id))?;

        media.update(update);

        Ok(())
    }
}

fn security_changes(rv: &mut Vec<ModifyTask>, existing: &HashMap<SecureKey, TaskSecurity>, new: &HashMap<SecureKey, TaskSecurity>) {
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
