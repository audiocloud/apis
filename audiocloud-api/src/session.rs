use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Range;
use std::str::FromStr;

use crate::change::{PlayId, RenderId};
use derive_more::{IsVariant, Unwrap};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

use crate::cloud::apps::{CreateSession, SessionSpec};
use crate::model::{MultiChannelTimestampedValue, MultiChannelValue};
use crate::newtypes::{DomainId, ReportId};
use crate::newtypes::{DynamicId, FixedId, FixedInstanceId, MediaId, MediaObjectId, MixerId, ModelId, ParameterId, SecureKey, TrackId};
use crate::time::TimeRange;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    pub domain_id: DomainId,
    pub time:      TimeRange,
    pub spec:      SessionSpec,
    pub security:  HashMap<SecureKey, SessionSecurity>,
    pub version:   u64,
}

impl From<CreateSession> for Session {
    fn from(source: CreateSession) -> Self {
        let CreateSession { time,
                            domain,
                            tracks,
                            mixers,
                            dynamic,
                            fixed,
                            security,
                            connections,
                            .. } = source;

        Self { domain_id: domain,

               time,
               security,
               version: 0,
               spec: SessionSpec { tracks,
                                   mixers,
                                   dynamic,
                                   fixed,
                                   connections } }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SessionMixer {
    pub input_channels:  usize,
    pub output_channels: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SessionDynamicInstance {
    pub model_id:   ModelId,
    pub parameters: InstanceParameters,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SessionFixedInstance {
    pub instance_id: FixedInstanceId,
    pub parameters:  InstanceParameters,
    pub wet:         f64, // only applicable for instances with <= 2 inputs and <= 2 outputs
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SessionConnection {
    pub from:          SessionFlowId,
    pub to:            SessionFlowId,
    pub from_channels: MixerChannels,
    pub to_channels:   MixerChannels,
    pub volume:        f64,
    pub pan:           f64,
}

pub type InstanceParameters = HashMap<ParameterId, MultiChannelValue>;
pub type InstanceReports = HashMap<ReportId, MultiChannelTimestampedValue>;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct ConnectionValues {
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
            MixerChannels::Mono(ch) => (1, ch),
            MixerChannels::Stereo(ch) => (2, ch),
        }
    }

    pub fn is_subset_of(self, range: Range<usize>) -> bool {
        match self {
            MixerChannels::Mono(ch) => range.contains(&ch),
            MixerChannels::Stereo(ch) => range.contains(&ch) && range.contains(&(ch + 1)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, IsVariant, Unwrap, Hash, Eq, PartialOrd, Ord)]
pub enum SessionFlowId {
    MixerInput(MixerId),
    MixerOutput(MixerId),
    FixedInstanceInput(FixedId),
    FixedInstanceOutput(FixedId),
    DynamicInstanceInput(DynamicId),
    DynamicInstanceOutput(DynamicId),
    TrackOutput(TrackId),
}

impl SessionFlowId {
    pub fn is_input(&self) -> bool {
        matches!(self,
                 SessionFlowId::MixerInput(_) | SessionFlowId::FixedInstanceInput(_) | SessionFlowId::DynamicInstanceInput(_))
    }

    pub fn is_output(&self) -> bool {
        matches!(self,
                 SessionFlowId::MixerOutput(_)
                 | SessionFlowId::FixedInstanceOutput(_)
                 | SessionFlowId::DynamicInstanceOutput(_)
                 | SessionFlowId::TrackOutput(_))
    }
}

impl std::fmt::Display for SessionFlowId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner_json = match serde_json::to_value(self).unwrap() {
            serde_json::Value::String(s) => s,
            _ => unreachable!(),
        };
        f.write_str(&inner_json)
    }
}

impl Serialize for SessionFlowId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&match self {
                                     SessionFlowId::MixerInput(mixer) => format!("mix:inp:{mixer}"),
                                     SessionFlowId::FixedInstanceInput(fixed) => format!("fix:inp:{fixed}"),
                                     SessionFlowId::DynamicInstanceInput(dynamic) => format!("dyn:inp:{dynamic}"),
                                     SessionFlowId::MixerOutput(mixer) => format!("mix:out:{mixer}"),
                                     SessionFlowId::FixedInstanceOutput(fixed) => format!("fix:out:{fixed}"),
                                     SessionFlowId::DynamicInstanceOutput(dynamic) => format!("dyn:out:{dynamic}"),
                                     SessionFlowId::TrackOutput(track) => format!("trk:out:{track}"),
                                 })
    }
}

impl<'de> Deserialize<'de> for SessionFlowId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let err = |msg| serde::de::Error::custom(msg);
        let string = String::deserialize(deserializer)?;
        let sep_pos = string.find(':').ok_or_else(|| err("expected separator ':'"))?;
        let sep_pos2 = string[(sep_pos + 1)..].find(':').ok_or_else(|| err("expected separator ':'"))?;
        let rest = string[(sep_pos + sep_pos2 + 2)..].to_owned();

        Ok(match (&string[..sep_pos], &string[(sep_pos + 1)..(sep_pos + sep_pos2 + 1)]) {
            ("mix", "inp") => Self::MixerInput(MixerId::new(rest)),
            ("mix", "out") => Self::MixerOutput(MixerId::new(rest)),
            ("fix", "inp") => Self::FixedInstanceInput(FixedId::new(rest)),
            ("fix", "out") => Self::FixedInstanceOutput(FixedId::new(rest)),
            ("dyn", "inp") => Self::DynamicInstanceInput(DynamicId::new(rest)),
            ("dyn", "out") => Self::DynamicInstanceOutput(DynamicId::new(rest)),
            ("trk", "out") => Self::TrackOutput(TrackId::new(rest)),
            (a, b) => return Err(err(&format!("unrecognized SessionFlowId variant: '{a}', '{b}'"))),
        })
    }
}

impl FromStr for SessionFlowId {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct  SessionTrack {
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
            SessionTrackChannels::Mono => 1,
            SessionTrackChannels::Stereo => 2,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SessionTrackMedia {
    pub channels:         SessionTrackChannels,
    pub media_segment:    SessionTimeSegment,
    pub timeline_segment: SessionTimeSegment,
    pub object_id:        MediaObjectId,
    pub format:           SessionTrackMediaFormat,
}

impl SessionTrackMedia {
    pub fn update(&mut self, update: UpdateSessionTrackMedia) {
        let UpdateSessionTrackMedia { channels,
                                      media_segment,
                                      timeline_segment,
                                      object_id, } = update;

        if let Some(channels) = channels {
            self.channels = channels;
        }

        if let Some(media_segment) = media_segment {
            self.media_segment = media_segment;
        }

        if let Some(timeline_segment) = timeline_segment {
            self.timeline_segment = timeline_segment;
        }

        if let Some(object_id) = object_id {
            self.object_id = object_id;
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct UpdateSessionTrackMedia {
    pub channels:         Option<SessionTrackChannels>,
    pub media_segment:    Option<SessionTimeSegment>,
    pub timeline_segment: Option<SessionTimeSegment>,
    pub object_id:        Option<MediaObjectId>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum SessionTrackMediaFormat {
    #[serde(rename = "wav")]
    Wav,
    #[serde(rename = "mp3")]
    Mp3,
    #[serde(rename = "flac")]
    Flac,
    #[serde(rename = "wavpack")]
    WavPack,
}

impl Display for SessionTrackMediaFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match serde_json::to_value(self).unwrap() {
            Value::String(s) => s,
            _ => unreachable!(),
        };
        f.write_str(&s)
    }
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

// The overall state of the session state machine
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SessionMode {
    StoppingRender(RenderId),
    StoppingPlay(PlayId),
    PreparingToPlay(PlayId),
    PreparingToRender(RenderId),
    Rendering(RenderId),
    Playing(PlayId),
    Idle,
}
