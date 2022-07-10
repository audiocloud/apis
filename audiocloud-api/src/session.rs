use std::collections::HashMap;

use derive_more::{Display, IsVariant, Unwrap};
use serde::{Deserialize, Serialize};

use crate::cloud::apps::CreateOrReplaceSession;
use crate::model::MultiChannelValue;
use crate::newtypes::DomainId;
use crate::newtypes::{AppId, DynamicId, FixedId, FixedInstanceId, InputId, MediaId, MediaObjectId, MixerId, ModelId, ParameterId, SecureKey, TrackId};
use crate::time::TimeRange;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Session {
  pub version:      u64,
  pub domain_id:    DomainId,
  pub domain_event: u64,
  pub app_id:       AppId,
  pub time:         TimeRange,
  pub tracks:       HashMap<TrackId, SessionTrack>,
  pub mixers:       HashMap<MixerId, SessionMixer>,
  pub dynamic:      HashMap<DynamicId, SessionDynamicInstance>,
  pub fixed:        HashMap<FixedId, SessionFixedInstance>,
  pub security:     HashMap<SecureKey, SessionSecurity>,
  pub deleted:      bool,
}

impl From<CreateOrReplaceSession> for Session {
  fn from(source: CreateOrReplaceSession) -> Self {
    let CreateOrReplaceSession { time,
                                 domain,
                                 app,
                                 tracks,
                                 mixers,
                                 dynamic,
                                 fixed,
                                 security,
                                 .. } = source;

    Self { version: 0,
           domain_id: domain,
           domain_event: 0,
           app_id: app,
           time,
           tracks,
           mixers,
           dynamic,
           fixed,
           security,
           deleted: false }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SessionMixer {
  pub channels: usize,
  pub inputs:   HashMap<InputId, MixerInput>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SessionDynamicInstance {
  pub model_id:   ModelId,
  pub parameters: InstanceParameters,
  pub inputs:     HashMap<InputId, MixerInput>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SessionFixedInstance {
  pub instance_id: FixedInstanceId,
  pub parameters:  InstanceParameters,
  pub inputs:      HashMap<InputId, MixerInput>,
}

pub type InstanceParameters = HashMap<ParameterId, MultiChannelValue>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MixerInput {
  pub source_id:      SessionObjectId,
  pub input_channels: MixerChannels,
  pub mixer_channels: MixerChannels,
  pub volume:         f64,
  pub pan:            f64,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct MixerInputValues {
  pub volume: Option<f64>,
  pub pan:    Option<f64>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, IsVariant, Unwrap)]
#[serde(rename_all = "snake_case")]
pub enum MixerChannels {
  Mono(usize),
  Stereo(usize),
}

impl MixerChannels {
  pub fn to_count_and_index(self) -> (usize, usize) {
    match self {
      | MixerChannels::Mono(ch) => (1, ch),
      | MixerChannels::Stereo(ch) => (2, ch),
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, IsVariant, Unwrap)]
#[serde(rename_all = "snake_case")]
pub enum SessionObjectId {
  Mixer(MixerId),
  FixedInstance(FixedId),
  DynamicInstance(DynamicId),
  Track(TrackId),
}

impl From<SessionMixerId> for SessionObjectId {
  fn from(m: SessionMixerId) -> Self {
    match m {
      | SessionMixerId::Mixer(id) => Self::Mixer(id),
      | SessionMixerId::FixedInstance(id) => Self::FixedInstance(id),
      | SessionMixerId::DynamicInstance(id) => Self::DynamicInstance(id),
    }
  }
}

impl Into<Option<SessionMixerId>> for SessionObjectId {
  fn into(self) -> Option<SessionMixerId> {
    Some(match self {
      | SessionObjectId::Mixer(id) => SessionMixerId::Mixer(id),
      | SessionObjectId::FixedInstance(id) => SessionMixerId::FixedInstance(id),
      | SessionObjectId::DynamicInstance(id) => SessionMixerId::DynamicInstance(id),
      | SessionObjectId::Track(_) => {
        return None;
      }
    })
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, IsVariant, Unwrap)]
#[serde(rename_all = "snake_case")]
pub enum SessionMixerId {
  Mixer(MixerId),
  FixedInstance(FixedId),
  DynamicInstance(DynamicId),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SessionTrack {
  pub channels: SessionTrackChannels,
  pub media:    HashMap<MediaId, SessionTrackMedia>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SessionTrackChannels {
  Mono,
  Stereo,
}

impl SessionTrackChannels {
  pub fn num_channels(&self) -> usize {
    match self {
      | SessionTrackChannels::Mono => 1,
      | SessionTrackChannels::Stereo => 2,
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SessionTrackMedia {
  pub channels:         SessionTrackChannels,
  pub media_segment:    SessionTimeSegment,
  pub timeline_segment: SessionTimeSegment,
  pub object_id:        MediaObjectId,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct SessionTimeSegment {
  pub start:  f64,
  pub length: f64,
}

impl SessionTimeSegment {
  pub fn end(&self) -> f64 {
    self.start + self.length
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SessionSecurity {
  pub structure:  bool,
  pub media:      bool,
  pub parameters: bool,
  pub transport:  bool,
  pub audio:      bool,
}