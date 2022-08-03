use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use derive_more::{Constructor, Display, From, Into};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::cloud::apps::SessionSpec;
use crate::instance::DesiredInstancePlayState;
use crate::model::MultiChannelValue;
use crate::newtypes::{DynamicId, FixedId, FixedInstanceId, InputId, MediaId, MediaObjectId, MixerId, ParameterId, SecureKey, TrackId};
use crate::session::{
    MixerInput, MixerInputValues, Session, SessionDynamicInstance, SessionFixedInstance, SessionMixer, SessionMixerId, SessionObjectId,
    SessionTimeSegment, SessionTrack, SessionTrackChannels, SessionTrackMedia,
};
use crate::session::{SessionMode, SessionSecurity};
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
        track_id:         TrackId,
        media_id:         MediaId,
        channels:         SessionTrackChannels,
        media_segment:    SessionTimeSegment,
        timeline_segment: SessionTimeSegment,
        object_id:        MediaObjectId,
    },
    SetTrackMediaValues {
        track_id:         TrackId,
        media_id:         MediaId,
        channels:         Option<SessionTrackChannels>,
        media_segment:    Option<SessionTimeSegment>,
        timeline_segment: Option<SessionTimeSegment>,
        object_id:        Option<MediaObjectId>,
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
        mixer_id: SessionMixerId,
    },
    DeleteMixerInput {
        mixer_id: SessionMixerId,
        input_id: InputId,
    },
    DeleteInputsReferencing {
        source_id: SessionObjectId,
    },
    AddMixerInput {
        mixer_id: SessionMixerId,
        input_id: InputId,
        input:    MixerInput,
    },
    SetInputValues {
        mixer_id: SessionMixerId,
        input_id: InputId,
        values:   MixerInputValues,
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
            ModifySessionSpec::SetTrackMediaValues { .. } => "set_track_media_values",
            ModifySessionSpec::DeleteTrackMedia { .. } => "delete_track_media",
            ModifySessionSpec::DeleteTrack { .. } => "delete_track",
            ModifySessionSpec::AddFixedInstance { .. } => "add_fixed_instance",
            ModifySessionSpec::AddDynamicInstance { .. } => "add_dynamic_instance",
            ModifySessionSpec::AddMixer { .. } => "add_mixer",
            ModifySessionSpec::DeleteMixer { .. } => "delete_mixer",
            ModifySessionSpec::DeleteMixerInput { .. } => "delete_mixer_input",
            ModifySessionSpec::DeleteInputsReferencing { .. } => "delete_inputs_referencing",
            ModifySessionSpec::AddMixerInput { .. } => "add_mixer_input",
            ModifySessionSpec::SetInputValues { .. } => "set_input_values",
            ModifySessionSpec::SetFixedInstanceParameterValues { .. } => "set_fixed_instance_parameter_values",
            ModifySessionSpec::SetDynamicInstanceParameterValues { .. } => "set_dynamic_instance_parameter_values",
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

    pub fn preparing(&self) -> SessionPlayState {
        match self {
            DesiredSessionPlayState::Play(play) => SessionPlayState::PreparingToPlay(play.clone()),
            DesiredSessionPlayState::Render(render) => SessionPlayState::PreparingToRender(render.clone()),
            DesiredSessionPlayState::Stopped => SessionPlayState::PreparingToStop,
        }
    }

    pub fn to_instance(&self) -> DesiredInstancePlayState {
        match self {
            DesiredSessionPlayState::Play(play) => DesiredInstancePlayState::Playing { play_id: play.play_id },
            DesiredSessionPlayState::Render(render) => DesiredInstancePlayState::Rendering { render_id: render.render_id,
                                                                                             length:    render.segment.length, },
            DesiredSessionPlayState::Stopped => DesiredInstancePlayState::Stopped,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlaySession {
    pub play_id:     PlayId,
    pub segment:     SessionTimeSegment,
    pub start_at:    f64,
    pub looping:     bool,
    pub sample_rate: SampleRate,
    pub bit_depth:   PlayBitDepth,
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
    pub object_id:  MediaObjectId,
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
    PreparingToStop,
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
    pub mode:               Timestamped<SessionMode>,
}

impl Default for SessionState {
    fn default() -> Self {
        Self { play_state:         Timestamped::new(SessionPlayState::Stopped),
               desired_play_state: Timestamped::new(DesiredSessionPlayState::Stopped),
               mode:               Timestamped::new(SessionMode::Idle), }
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

    #[error("Input {1} of {0:?} already exists")]
    InputExists(SessionMixerId, InputId),
    #[error("Input {1} of {0:?} does not exist")]
    InputDoesNotExist(SessionMixerId, InputId),

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

    pub fn modify(&mut self, modify: ModifySessionSpec) -> Result<(), ModifySessionError> {
        match modify {
            ModifySessionSpec::AddFixedInstance { fixed_id: mixer_id,
                                                  process, } => self.add_fixed_instance(mixer_id, process),
            ModifySessionSpec::AddDynamicInstance { dynamic_id: mixer_id,
                                                    process, } => self.add_dynamic_instance(mixer_id, process),
            ModifySessionSpec::AddMixer { mixer_id, mixer: channels } => self.add_mixer(mixer_id, channels),
            ModifySessionSpec::DeleteMixer { mixer_id } => self.delete_mixer(mixer_id),
            ModifySessionSpec::DeleteMixerInput { mixer_id, input_id } => self.delete_mixer_input(mixer_id, input_id),
            ModifySessionSpec::AddMixerInput { mixer_id, input_id, input } => self.add_mixer_input(mixer_id, input_id, input),
            ModifySessionSpec::SetFixedInstanceParameterValues { fixed_id: id, values } => {
                self.set_fixed_instance_parameter_values(id, values)
            }

            ModifySessionSpec::SetDynamicInstanceParameterValues { dynamic_id: id, values } => {
                self.set_dynamic_instance_parameter_values(id, values)
            }
            ModifySessionSpec::DeleteInputsReferencing { source_id } => self.delete_inputs_referencing(source_id),
            ModifySessionSpec::AddTrack { track_id, channels } => self.add_track(track_id, channels),
            ModifySessionSpec::DeleteTrackMedia { track_id, media_id } => self.delete_track_media(track_id, media_id),
            ModifySessionSpec::DeleteTrack { track_id } => self.delete_track(track_id),
            ModifySessionSpec::SetInputValues { mixer_id,
                                                input_id,
                                                values, } => self.set_input_values(mixer_id, input_id, values),
            ModifySessionSpec::AddTrackMedia { track_id,
                                               media_id,
                                               channels,
                                               media_segment,
                                               timeline_segment,
                                               object_id, } => {
                self.add_track_media(track_id, media_id, channels, media_segment, timeline_segment, object_id)
            }
            ModifySessionSpec::SetTrackMediaValues { track_id,
                                                     media_id,
                                                     channels,
                                                     media_segment,
                                                     timeline_segment,
                                                     object_id, } => {
                self.set_track_media_values(track_id, media_id, channels, media_segment, timeline_segment, object_id)
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

    pub fn delete_mixer(&mut self, mixer_id: SessionMixerId) -> Result<(), ModifySessionError> {
        use SessionObjectId::*;

        match mixer_id {
            SessionMixerId::Mixer(id) => {
                if self.mixers.remove(&id).is_none() {
                    Err(MixerDoesNotExist(id))
                } else {
                    self.delete_inputs_referencing(Mixer(id))
                }
            }
            SessionMixerId::FixedInstance(id) => {
                if self.fixed.remove(&id).is_none() {
                    Err(FixedInstanceDoesNotExist(id))
                } else {
                    self.delete_inputs_referencing(FixedInstance(id))
                }
            }
            SessionMixerId::DynamicInstance(id) => {
                if self.dynamic.remove(&id).is_none() {
                    Err(DynamicInstanceDoesNotExist(id))
                } else {
                    self.delete_inputs_referencing(DynamicInstance(id))
                }
            }
        }
    }

    pub fn delete_mixer_input(&mut self, mixer_id: SessionMixerId, input_id: InputId) -> Result<(), ModifySessionError> {
        match &mixer_id {
            SessionMixerId::Mixer(mixer) => {
                let mixer = self.mixers.get_mut(&mixer).ok_or(MixerDoesNotExist(mixer.clone()))?;
                Self::delete_input(&mut mixer.inputs, &mixer_id, input_id)
            }
            SessionMixerId::FixedInstance(mixer) => {
                let mixer = self.fixed.get_mut(&mixer).ok_or(FixedInstanceDoesNotExist(mixer.clone()))?;
                Self::delete_input(&mut mixer.inputs, &mixer_id, input_id)
            }
            SessionMixerId::DynamicInstance(mixer) => {
                let mixer = self.dynamic.get_mut(&mixer).ok_or(DynamicInstanceDoesNotExist(mixer.clone()))?;
                Self::delete_input(&mut mixer.inputs, &mixer_id, input_id)
            }
        }
    }

    fn delete_input(inputs: &mut HashMap<InputId, MixerInput>,
                    mixer_id: &SessionMixerId,
                    input_id: InputId)
                    -> Result<(), ModifySessionError> {
        match inputs.remove(&input_id) {
            None => Err(InputDoesNotExist(mixer_id.clone(), input_id)),
            Some(_) => Ok(()),
        }
    }

    pub fn add_mixer_input(&mut self, mixer_id: SessionMixerId, input_id: InputId, input: MixerInput) -> Result<(), ModifySessionError> {
        match &input.source_id {
            SessionObjectId::Mixer(id) if !self.mixers.contains_key(&id) => Err(MixerDoesNotExist(id.clone())),
            SessionObjectId::FixedInstance(id) if !self.fixed.contains_key(&id) => Err(FixedInstanceDoesNotExist(id.clone())),
            SessionObjectId::DynamicInstance(id) if !self.dynamic.contains_key(&id) => Err(DynamicInstanceDoesNotExist(id.clone())),
            SessionObjectId::Track(id) if !self.tracks.contains_key(&id) => Err(TrackDoesNotExist(id.clone())),
            _ => Ok(()),
        }?;

        if let Some(source_mixer_id) = input.source_id.clone().into() {
            if self.is_connected(&mixer_id, &source_mixer_id) {
                return Err(CycleDetected);
            }
        }

        match &mixer_id {
            SessionMixerId::Mixer(mixer) => {
                let mixer = self.mixers.get_mut(&mixer).ok_or(MixerDoesNotExist(mixer.clone()))?;
                Self::add_input(&mut mixer.inputs, &mixer_id, &input_id, input)
            }
            SessionMixerId::FixedInstance(mixer) => {
                let mixer = self.fixed.get_mut(&mixer).ok_or(FixedInstanceDoesNotExist(mixer.clone()))?;
                Self::add_input(&mut mixer.inputs, &mixer_id, &input_id, input)
            }
            SessionMixerId::DynamicInstance(mixer) => {
                let mixer = self.dynamic.get_mut(&mixer).ok_or(DynamicInstanceDoesNotExist(mixer.clone()))?;
                Self::add_input(&mut mixer.inputs, &mixer_id, &input_id, input)
            }
        }
    }

    pub fn is_connected(&self, from: &SessionMixerId, to: &SessionMixerId) -> bool {
        if let Some(maybe_inputs) = match to {
            SessionMixerId::Mixer(m) => self.mixers.get(&m).map(|m| &m.inputs),
            SessionMixerId::FixedInstance(m) => self.fixed.get(&m).map(|m| &m.inputs),
            SessionMixerId::DynamicInstance(m) => self.dynamic.get(&m).map(|m| &m.inputs),
        } {
            for input in maybe_inputs.values() {
                if let Some(source_mixer_id) = input.source_id.clone().into() {
                    if from == &source_mixer_id {
                        return true;
                    }

                    if self.is_connected(from, &source_mixer_id) {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn add_input(output: &mut HashMap<InputId, MixerInput>,
                 mixer_id: &SessionMixerId,
                 input_id: &InputId,
                 input: MixerInput)
                 -> Result<(), ModifySessionError> {
        if output.contains_key(&input_id) {
            Err(InputExists(mixer_id.clone(), input_id.clone()))
        } else {
            output.insert(input_id.clone(), input);
            Ok(())
        }
    }

    pub fn set_input_values(&mut self,
                            mixer_id: SessionMixerId,
                            input_id: InputId,
                            values: MixerInputValues)
                            -> Result<(), ModifySessionError> {
        let inputs = match &mixer_id {
            SessionMixerId::Mixer(m) => self.mixers.get_mut(&m).map(|m| &mut m.inputs).ok_or(MixerDoesNotExist(m.clone()))?,
            SessionMixerId::FixedInstance(m) => self.fixed
                                                    .get_mut(&m)
                                                    .map(|m| &mut m.inputs)
                                                    .ok_or(FixedInstanceDoesNotExist(m.clone()))?,
            SessionMixerId::DynamicInstance(m) => self.dynamic
                                                      .get_mut(&m)
                                                      .map(|m| &mut m.inputs)
                                                      .ok_or(DynamicInstanceDoesNotExist(m.clone()))?,
        };

        if let Some(input) = inputs.get_mut(&input_id) {
            if let Some(volume) = values.volume {
                input.volume = volume;
            }
            if let Some(pan) = values.pan {
                input.pan = pan;
            }

            Ok(())
        } else {
            Err(InputDoesNotExist(mixer_id.clone(), input_id))
        }
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

    pub fn delete_inputs_referencing(&mut self, source_id: SessionObjectId) -> Result<(), ModifySessionError> {
        for mixer in self.mixers.values_mut() {
            Self::delete_inputs_referencing_from_map(&mut mixer.inputs, &source_id);
        }

        for fixed in self.fixed.values_mut() {
            Self::delete_inputs_referencing_from_map(&mut fixed.inputs, &source_id);
        }

        for dynamic in self.dynamic.values_mut() {
            Self::delete_inputs_referencing_from_map(&mut dynamic.inputs, &source_id);
        }

        Ok(())
    }

    fn delete_inputs_referencing_from_map(inputs: &mut HashMap<InputId, MixerInput>, source_id: &SessionObjectId) {
        inputs.retain(|_, v| &v.source_id != source_id);
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

    pub fn add_track_media(&mut self,
                           track_id: TrackId,
                           media_id: MediaId,
                           channels: SessionTrackChannels,
                           media_segment: SessionTimeSegment,
                           timeline_segment: SessionTimeSegment,
                           object_id: MediaObjectId)
                           -> Result<(), ModifySessionError> {
        let track = self.tracks.get_mut(&track_id).ok_or(TrackDoesNotExist(track_id.clone()))?;

        if track.media.contains_key(&media_id) {
            return Err(MediaDoesNotExist(track_id.clone(), media_id));
        }

        track.media.insert(media_id,
                           SessionTrackMedia { channels,
                                               media_segment,
                                               timeline_segment,
                                               object_id });

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
            self.delete_inputs_referencing(SessionObjectId::Track(track_id))
        } else {
            Err(TrackDoesNotExist(track_id))
        }
    }

    pub fn set_track_media_values(&mut self,
                                  track_id: TrackId,
                                  media_id: MediaId,
                                  channels: Option<SessionTrackChannels>,
                                  media_segment: Option<SessionTimeSegment>,
                                  timeline_segment: Option<SessionTimeSegment>,
                                  object_id: Option<MediaObjectId>)
                                  -> Result<(), ModifySessionError> {
        let track = self.tracks.get_mut(&track_id).ok_or(TrackDoesNotExist(track_id.clone()))?;
        let media = track.media
                         .get_mut(&media_id)
                         .ok_or(MediaDoesNotExist(track_id.clone(), media_id))?;

        if let Some(channels) = channels {
            media.channels = channels;
        }

        if let Some(media_segment) = media_segment {
            media.media_segment = media_segment;
        }

        if let Some(timeline_segment) = timeline_segment {
            media.timeline_segment = timeline_segment;
        }

        if let Some(object_id) = object_id {
            media.object_id = object_id;
        }

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
