use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use derive_more::{Constructor, Display, From, Into};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::cloud::apps::SessionSpec;
use crate::cloud::media::AppMedia;
use crate::model::MultiChannelValue;
use crate::newtypes::{
    AppId, AppMediaObjectId, ConnectionId, DynamicId, FixedId, FixedInstanceId, MediaId, MediaObjectId, MixerId, ParameterId, SecureKey,
    TrackId,
};
use crate::session::SessionSecurity;
use crate::session::{
    ConnectionValues, MixerChannels, Session, SessionConnection, SessionDynamicInstance, SessionFixedInstance, SessionFlowId, SessionMixer,
    SessionTimeSegment, SessionTrack, SessionTrackChannels, SessionTrackMedia, UpdateSessionTrackMedia,
};
use crate::time::Timestamped;

use self::ModifySessionError::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ModifySessionSpec {
    AddTrack {
        track_id: TrackId,
        channels: SessionTrackChannels,
    },
    AddTrackMedia {
        track_id: TrackId,
        media_id: MediaId,
        spec:     SessionTrackMedia,
    },
    UpdateTrackMedia {
        track_id: TrackId,
        media_id: MediaId,
        update:   UpdateSessionTrackMedia,
    },
    DeleteTrackMedia {
        track_id: TrackId,
        media_id: MediaId,
    },
    DeleteTrack {
        track_id: TrackId,
    },
    AddFixedInstance {
        fixed_id: FixedId,
        process:  SessionFixedInstance,
    },
    AddDynamicInstance {
        dynamic_id: DynamicId,
        process:    SessionDynamicInstance,
    },
    AddMixer {
        mixer_id: MixerId,
        mixer:    SessionMixer,
    },
    DeleteMixer {
        mixer_id: MixerId,
    },
    DeleteFixedInstance {
        fixed_id: FixedId,
    },
    DeleteDynamicInstance {
        dynamic_id: DynamicId,
    },
    DeleteConnection {
        connection_id: ConnectionId,
    },
    AddConnection {
        connection_id: ConnectionId,
        from:          SessionFlowId,
        to:            SessionFlowId,
        from_channels: MixerChannels,
        to_channels:   MixerChannels,
        volume:        f64,
        pan:           f64,
    },
    SetConnectionParameterValues {
        connection_id: ConnectionId,
        values:        ConnectionValues,
    },
    SetFixedInstanceParameterValues {
        fixed_id: FixedId,
        values:   HashMap<ParameterId, MultiChannelValue>,
    },
    SetDynamicInstanceParameterValues {
        dynamic_id: DynamicId,
        values:     HashMap<ParameterId, MultiChannelValue>,
    },
}

impl ModifySessionSpec {
    pub fn get_kind(&self) -> &'static str {
        match self {
            ModifySessionSpec::AddTrack { .. } => "add_track",
            ModifySessionSpec::AddTrackMedia { .. } => "add_track_media",
            ModifySessionSpec::UpdateTrackMedia { .. } => "update_track_media",
            ModifySessionSpec::DeleteTrackMedia { .. } => "delete_track_media",
            ModifySessionSpec::DeleteTrack { .. } => "delete_track",
            ModifySessionSpec::AddFixedInstance { .. } => "add_fixed_instance",
            ModifySessionSpec::AddDynamicInstance { .. } => "add_dynamic_instance",
            ModifySessionSpec::AddMixer { .. } => "add_mixer",
            ModifySessionSpec::DeleteMixer { .. } => "delete_mixer",
            ModifySessionSpec::AddConnection { .. } => "add_mixer_input",
            ModifySessionSpec::SetConnectionParameterValues { .. } => "set_input_values",
            ModifySessionSpec::SetFixedInstanceParameterValues { .. } => "set_fixed_instance_parameter_values",
            ModifySessionSpec::SetDynamicInstanceParameterValues { .. } => "set_dynamic_instance_parameter_values",
            ModifySessionSpec::DeleteFixedInstance { .. } => "delete_fixed_instance",
            ModifySessionSpec::DeleteDynamicInstance { .. } => "delete_dynamic_instance",
            ModifySessionSpec::DeleteConnection { .. } => "delete_connection",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ModifySession {
    Spec(ModifySessionSpec),
    SetSecurity { key: SecureKey, security: SessionSecurity },
    RevokeSecurity { key: SecureKey },
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SampleRate {
    #[serde(rename = "192")]
    SR192,
    #[serde(rename = "96")]
    SR96,
    #[serde(rename = "88.2")]
    SR88_2,
    #[serde(rename = "48")]
    SR48,
    #[serde(rename = "44.1")]
    SR44_1,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PlayBitDepth {
    #[serde(rename = "24")]
    PD24,
    #[serde(rename = "16")]
    PD16,
}

impl Into<usize> for PlayBitDepth {
    fn into(self) -> usize {
        match self {
            PlayBitDepth::PD24 => 24,
            PlayBitDepth::PD16 => 16,
        }
    }
}

impl Into<usize> for SampleRate {
    fn into(self) -> usize {
        match self {
            SampleRate::SR192 => 192_000,
            SampleRate::SR96 => 96_000,
            SampleRate::SR88_2 => 88_200,
            SampleRate::SR48 => 48_000,
            SampleRate::SR44_1 => 44_100,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DesiredSessionPlayState {
    /// Play, with sample rate conversion
    Play(PlaySession),

    /// Rendering is always a F32 WAV at full sample rate, so nothing else needs to happen here
    Render(RenderSession),
    Stopped,
}

impl DesiredSessionPlayState {
    pub fn is_stop(&self) -> bool {
        self == &Self::Stopped
    }

    pub fn is_render(&self) -> bool {
        matches!(self, Self::Render(_))
    }

    pub fn is_rendering_of(&self, render: &RenderSession) -> bool {
        matches!(self, DesiredSessionPlayState::Render(desired_render) if desired_render == render)
    }

    pub fn is_playing_of(&self, play: &PlaySession) -> bool {
        matches!(self, DesiredSessionPlayState::Play(desired_play) if desired_play == play)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlaySession {
    pub play_id:     PlayId,
    pub mixer_id:    MixerId,
    pub segment:     SessionTimeSegment,
    pub start_at:    f64,
    pub looping:     bool,
    pub sample_rate: SampleRate,
    pub bit_depth:   PlayBitDepth,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UpdatePlaySession {
    pub play_id:  PlayId,
    pub mixer_id: Option<MixerId>,
    pub segment:  Option<SessionTimeSegment>,
    pub start_at: Option<f64>,
    pub looping:  bool,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct PlaySegment {
    pub segment:  SessionTimeSegment,
    pub looping:  bool,
    pub start_at: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RenderSession {
    pub render_id:  RenderId,
    pub mixer_id:   MixerId,
    pub segment:    SessionTimeSegment,
    pub object_id:  AppMediaObjectId,
    pub put_url:    String,
    pub notify_url: String,
    pub context:    String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SuccessfulRenderNotification {
    pub render_id: RenderId,
    pub object_id: MediaObjectId,
    pub context:   String,
}

pub type RenderNotification = Result<SuccessfulRenderNotification, String>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SessionPlayState {
    PreparingToPlay(PlaySession),
    PreparingToRender(RenderSession),
    Playing(PlaySession),
    Rendering(RenderSession),
    StoppingPlay(PlayId),
    StoppingRender(RenderId),
    Stopped,
}

impl SessionPlayState {
    pub fn is_playing(&self, play_id: PlayId) -> bool {
        matches!(self, Self::Playing(playing) if playing.play_id == play_id)
    }

    pub fn is_rendering(&self, render_id: RenderId) -> bool {
        matches!(self, Self::Rendering(rendering) if rendering.render_id == render_id)
    }

    pub fn is_stopped(&self) -> bool {
        matches!(self, Self::Stopped)
    }

    pub fn satisfies(&self, desired: &DesiredSessionPlayState) -> bool {
        match (self, desired) {
            (Self::Playing(playing), DesiredSessionPlayState::Play(desired_playing)) => playing == desired_playing,
            (Self::Rendering(rendering), DesiredSessionPlayState::Render(desired_rendering)) => rendering == desired_rendering,
            (Self::Stopped, DesiredSessionPlayState::Stopped) => true,
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SessionState {
    pub play_state:         Timestamped<SessionPlayState>,
    pub desired_play_state: Timestamped<DesiredSessionPlayState>,
}

impl Default for SessionState {
    fn default() -> Self {
        Self { play_state:         Timestamped::new(SessionPlayState::Stopped),
               desired_play_state: Timestamped::new(DesiredSessionPlayState::Stopped), }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug, From, Into, Hash, Display, Constructor)]
#[repr(transparent)]
pub struct PlayId(u64);

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug, From, Into, Hash, Display, Constructor)]
#[repr(transparent)]
pub struct RenderId(u64);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Error)]
#[serde(rename_all = "snake_case")]
pub enum ModifySessionError {
    #[error("Track {0} already exists")]
    TrackExists(TrackId),
    #[error("Fixed instance {0} already exists")]
    FixedInstanceExists(FixedId),
    #[error("Dynamic instance {0} already exists")]
    DynamicInstanceExists(DynamicId),
    #[error("Mixer {0} already exists")]
    MixerExists(MixerId),

    #[error("Track {0} does not exist")]
    TrackDoesNotExist(TrackId),
    #[error("Fixed instance {0} does not exist")]
    FixedInstanceDoesNotExist(FixedId),
    #[error("Dynamic instance {0} does not exist")]
    DynamicInstanceDoesNotExist(DynamicId),
    #[error("Mixer {0} does not exist")]
    MixerDoesNotExist(MixerId),
    #[error("Connection {0} does not exist")]
    ConnectionDoesNotExist(ConnectionId),
    #[error("Connection {0} already exist")]
    ConnectionExists(ConnectionId),
    #[error("Connection {0} already exist: {1}")]
    ConnectionMalformed(ConnectionId, String),

    #[error("Media {1} on track {0} already exists")]
    MediaExists(TrackId, MediaId),
    #[error("Media {1} on track {0} does not exist")]
    MediaDoesNotExist(TrackId, MediaId),

    #[error("Refusing to add connection - cycle detected")]
    CycleDetected,
}

impl Session {
    pub fn get_media_object_ids<'a>(&'a self) -> impl Iterator<Item = &'a MediaObjectId> + 'a {
        self.spec
            .tracks
            .values()
            .flat_map(|track| track.media.values().map(|media| &media.object_id))
    }

    pub fn generate_changes(&self, other: &Session) -> Vec<ModifySession> {
        let mut rv = vec![];
        security_changes(&mut rv, &self.security, &other.security);

        rv
    }

    pub fn apply_change(&mut self, modify: ModifySession) -> Result<(), ModifySessionError> {
        match modify {
            ModifySession::Spec(spec_change) => {
                self.spec.modify(spec_change)?;
            }
            ModifySession::SetSecurity { key, security } => {
                self.set_security(key, security)?;
            }
            ModifySession::RevokeSecurity { key } => {
                self.revoke_security(key)?;
            }
        }

        Ok(())
    }

    pub fn set_security(&mut self, key: SecureKey, security: SessionSecurity) -> Result<(), ModifySessionError> {
        self.security.insert(key, security);
        Ok(())
    }

    pub fn revoke_security(&mut self, key: SecureKey) -> Result<(), ModifySessionError> {
        self.security.remove(&key);
        Ok(())
    }
}

impl SessionSpec {
    pub fn get_fixed_instance_ids<'a>(&'a self) -> impl Iterator<Item = &'a FixedInstanceId> + 'a {
        self.fixed.values().map(|fixed| &fixed.instance_id)
    }

    pub fn get_media_object_ids(&self, app_id: &AppId) -> HashSet<AppMediaObjectId> {
        self.tracks
            .values()
            .flat_map(|track| track.media.values().map(|media| media.object_id.clone().for_app(app_id.clone())))
            .collect()
    }

    pub fn modify(&mut self, modify: ModifySessionSpec) -> Result<(), ModifySessionError> {
        match modify {
            ModifySessionSpec::AddFixedInstance { fixed_id: mixer_id,
                                                  process, } => self.add_fixed_instance(mixer_id, process),
            ModifySessionSpec::AddDynamicInstance { dynamic_id: mixer_id,
                                                    process, } => self.add_dynamic_instance(mixer_id, process),
            ModifySessionSpec::AddMixer { mixer_id, mixer: channels } => self.add_mixer(mixer_id, channels),
            ModifySessionSpec::DeleteMixer { mixer_id } => self.delete_mixer(mixer_id),
            ModifySessionSpec::SetFixedInstanceParameterValues { fixed_id: id, values } => {
                self.set_fixed_instance_parameter_values(id, values)
            }

            ModifySessionSpec::SetDynamicInstanceParameterValues { dynamic_id: id, values } => {
                self.set_dynamic_instance_parameter_values(id, values)
            }
            ModifySessionSpec::AddTrack { track_id, channels } => self.add_track(track_id, channels),
            ModifySessionSpec::DeleteTrackMedia { track_id, media_id } => self.delete_track_media(track_id, media_id),
            ModifySessionSpec::DeleteTrack { track_id } => self.delete_track(track_id),
            ModifySessionSpec::SetConnectionParameterValues { connection_id, values } => {
                self.set_connection_parameter_values(connection_id, values)
            }
            ModifySessionSpec::AddTrackMedia { track_id, media_id, spec } => self.add_track_media(track_id, media_id, spec),
            ModifySessionSpec::UpdateTrackMedia { track_id,
                                                  media_id,
                                                  update, } => self.update_track_media(track_id, media_id, update),
            ModifySessionSpec::DeleteFixedInstance { fixed_id } => self.delete_fixed_instance(fixed_id),
            ModifySessionSpec::DeleteDynamicInstance { dynamic_id } => self.delete_dynamic_instance(dynamic_id),
            ModifySessionSpec::DeleteConnection { connection_id } => self.delete_connection(connection_id),
            ModifySessionSpec::AddConnection { connection_id,
                                               from,
                                               to,
                                               from_channels,
                                               to_channels,
                                               volume,
                                               pan, } => {
                self.add_connection(connection_id, from, to, from_channels, to_channels, volume, pan)
            }
        }
    }

    pub fn add_fixed_instance(&mut self, fixed_id: FixedId, instance: SessionFixedInstance) -> Result<(), ModifySessionError> {
        if self.fixed.contains_key(&fixed_id) {
            return Err(FixedInstanceExists(fixed_id));
        }

        self.fixed.insert(fixed_id, instance);

        Ok(())
    }

    pub fn add_dynamic_instance(&mut self, dynamic_id: DynamicId, dynamic: SessionDynamicInstance) -> Result<(), ModifySessionError> {
        if self.dynamic.contains_key(&dynamic_id) {
            return Err(DynamicInstanceExists(dynamic_id));
        }

        self.dynamic.insert(dynamic_id, dynamic);

        Ok(())
    }

    pub fn add_mixer(&mut self, mixer_id: MixerId, mixer: SessionMixer) -> Result<(), ModifySessionError> {
        if self.mixers.contains_key(&mixer_id) {
            return Err(MixerExists(mixer_id));
        }

        self.mixers.insert(mixer_id, mixer);

        Ok(())
    }

    pub fn delete_mixer(&mut self, mixer_id: MixerId) -> Result<(), ModifySessionError> {
        if !self.mixers.contains_key(&mixer_id) {
            return Err(MixerDoesNotExist(mixer_id));
        }

        self.mixers.remove(&mixer_id);

        Ok(())
    }

    pub fn is_connected(&self, from: &SessionFlowId, to: &SessionFlowId) -> bool {
        self.connections
            .iter()
            .any(|(_, connection)| &connection.from == from && &connection.to == to)
    }

    pub fn set_connection_parameter_values(&mut self,
                                           connection_id: ConnectionId,
                                           values: ConnectionValues)
                                           -> Result<(), ModifySessionError> {
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
                                               fixed_id: FixedId,
                                               parameters: HashMap<ParameterId, MultiChannelValue>)
                                               -> Result<(), ModifySessionError> {
        let fixed = self.fixed.get_mut(&fixed_id).ok_or(FixedInstanceDoesNotExist(fixed_id))?;
        fixed.parameters.extend(parameters.into_iter());
        Ok(())
    }

    pub fn set_dynamic_instance_parameter_values(&mut self,
                                                 dynamic_id: DynamicId,
                                                 parameters: HashMap<ParameterId, MultiChannelValue>)
                                                 -> Result<(), ModifySessionError> {
        let dynamic = self.dynamic.get_mut(&dynamic_id).ok_or(DynamicInstanceDoesNotExist(dynamic_id))?;
        dynamic.parameters.extend(parameters.into_iter());
        Ok(())
    }

    pub fn delete_connections_referencing(&mut self, flow_id: SessionFlowId) {
        self.connections.retain(|_, value| &value.from != &flow_id && &value.to != &flow_id);
    }

    pub fn add_track(&mut self, track_id: TrackId, channels: SessionTrackChannels) -> Result<(), ModifySessionError> {
        if self.tracks.contains_key(&track_id) {
            return Err(TrackExists(track_id));
        }

        self.tracks.insert(track_id,
                           SessionTrack { channels,
                                          media: Default::default() });

        Ok(())
    }

    pub fn add_track_media(&mut self, track_id: TrackId, media_id: MediaId, spec: SessionTrackMedia) -> Result<(), ModifySessionError> {
        let track = self.tracks.get_mut(&track_id).ok_or(TrackDoesNotExist(track_id.clone()))?;

        if track.media.contains_key(&media_id) {
            return Err(MediaDoesNotExist(track_id.clone(), media_id));
        }

        track.media.insert(media_id, spec);

        Ok(())
    }

    pub fn delete_track_media(&mut self, track_id: TrackId, media_id: MediaId) -> Result<(), ModifySessionError> {
        let track = self.tracks.get_mut(&track_id).ok_or(TrackDoesNotExist(track_id.clone()))?;
        if track.media.remove(&media_id).is_none() {
            Err(MediaDoesNotExist(track_id.clone(), media_id))
        } else {
            Ok(())
        }
    }

    pub fn delete_track(&mut self, track_id: TrackId) -> Result<(), ModifySessionError> {
        if self.tracks.remove(&track_id).is_some() {
            self.delete_connections_referencing(SessionFlowId::TrackOutput(track_id));

            Ok(())
        } else {
            Err(TrackDoesNotExist(track_id))
        }
    }

    pub fn delete_fixed_instance(&mut self, fixed_id: FixedId) -> Result<(), ModifySessionError> {
        if self.fixed.remove(&fixed_id).is_some() {
            self.delete_connections_referencing(SessionFlowId::FixedInstanceOutput(fixed_id.clone()));
            self.delete_connections_referencing(SessionFlowId::FixedInstanceInput(fixed_id.clone()));

            Ok(())
        } else {
            Err(FixedInstanceDoesNotExist(fixed_id))
        }
    }

    pub fn delete_dynamic_instance(&mut self, dynamic_id: DynamicId) -> Result<(), ModifySessionError> {
        if self.dynamic.remove(&dynamic_id).is_some() {
            self.delete_connections_referencing(SessionFlowId::DynamicInstanceOutput(dynamic_id.clone()));
            self.delete_connections_referencing(SessionFlowId::DynamicInstanceInput(dynamic_id.clone()));

            Ok(())
        } else {
            Err(DynamicInstanceDoesNotExist(dynamic_id))
        }
    }

    pub fn delete_connection(&mut self, connection_id: ConnectionId) -> Result<(), ModifySessionError> {
        if self.connections.remove(&connection_id).is_some() {
            Ok(())
        } else {
            Err(ConnectionDoesNotExist(connection_id))
        }
    }

    pub fn add_connection(&mut self,
                          connection_id: ConnectionId,
                          from: SessionFlowId,
                          to: SessionFlowId,
                          from_channels: MixerChannels,
                          to_channels: MixerChannels,
                          volume: f64,
                          pan: f64)
                          -> Result<(), ModifySessionError> {
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
                                SessionConnection { from,
                                                    to,
                                                    from_channels,
                                                    to_channels,
                                                    volume,
                                                    pan });
        Ok(())
    }

    pub fn update_track_media(&mut self,
                              track_id: TrackId,
                              media_id: MediaId,
                              update: UpdateSessionTrackMedia)
                              -> Result<(), ModifySessionError> {
        let track = self.tracks.get_mut(&track_id).ok_or(TrackDoesNotExist(track_id.clone()))?;
        let media = track.media
                         .get_mut(&media_id)
                         .ok_or(MediaDoesNotExist(track_id.clone(), media_id))?;

        media.update(update);

        Ok(())
    }
}

fn security_changes(rv: &mut Vec<ModifySession>,
                    existing: &HashMap<SecureKey, SessionSecurity>,
                    new: &HashMap<SecureKey, SessionSecurity>) {
    let changes = hashmap_changes(existing, new);
    for (key, security) in changes.changed.into_iter().chain(changes.added.into_iter()) {
        rv.push(ModifySession::SetSecurity { key, security })
    }
    for key in changes.removed {
        rv.push(ModifySession::RevokeSecurity { key });
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
