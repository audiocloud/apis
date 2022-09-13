use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::ops::Range;
use std::str::FromStr;

use derive_more::{IsVariant, Unwrap};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

use crate::cloud::tasks::CreateTask;
use crate::cloud::CloudError;
use crate::cloud::CloudError::InternalInconsistency;
use crate::json_schema_new_type;
use crate::time::TimeRange;
use crate::{
    DomainId, DynamicInstanceNodeId, FixedInstanceId, FixedInstanceNodeId, MediaObjectId, MixerNodeId, Model, ModelId,
    MultiChannelTimestampedValue, NodeConnectionId, ReportId, SecureKey, TrackMediaId, TrackNodeId,
};

/// Task specification
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, JsonSchema)]
pub struct TaskSpec {
    /// Track nodes of the task
    #[serde(default)]
    pub tracks:      HashMap<TrackNodeId, TrackNode>,
    /// Mixer nodes of the task
    #[serde(default)]
    pub mixers:      HashMap<MixerNodeId, MixerNode>,
    /// Dynamic instance nodes of the task
    #[serde(default)]
    pub dynamic:     HashMap<DynamicInstanceNodeId, DynamicInstanceNode>,
    /// Fixed instance nodes of the task
    #[serde(default)]
    pub fixed:       HashMap<FixedInstanceNodeId, FixedInstanceNode>,
    /// Connections between nodes
    #[serde(default)]
    pub connections: HashMap<NodeConnectionId, NodeConnection>,
}

impl TaskSpec {
    pub fn validate(&self, models: &HashMap<ModelId, Model>) -> Result<(), CloudError> {
        if self.fixed.is_empty() && self.dynamic.is_empty() && self.mixers.is_empty() && self.tracks.is_empty() {
            return Err(InternalInconsistency { message:
                                                   format!("No tracks, mixers, dynamic instances, or fixed instances declared in task spec"), });
        }

        for (connection_id, connection) in self.connections.iter() {
            self.validate_connection(connection_id, connection, models)?;
        }

        Ok(())
    }

    pub fn fixed_instance_to_fixed_id(&self, instance_id: &FixedInstanceId) -> Option<&FixedInstanceNodeId> {
        for (fixed_id, fixed) in &self.fixed {
            if &fixed.instance_id == instance_id {
                return Some(fixed_id);
            }
        }
        None
    }

    fn validate_connection(&self,
                           id: &NodeConnectionId,
                           connection: &NodeConnection,
                           models: &HashMap<ModelId, Model>)
                           -> Result<(), CloudError> {
        let to = &connection.to;
        let from = &connection.from;

        if !from.is_output() {
            return Err(InternalInconsistency { message: format!("Connection {id} flow from {from} is not an output"), });
        }

        if !to.is_input() {
            return Err(InternalInconsistency { message: format!("Connection {id} flow to {to} is not an input"), });
        }

        self.check_channel_exists(id, &connection.from, &connection.from_channels, models)?;
        self.check_channel_exists(id, &connection.to, &connection.to_channels, models)?;

        Ok(())
    }

    fn check_channel_exists(&self,
                            id: &NodeConnectionId,
                            flow_id: &NodePadId,
                            channels: &ChannelMask,
                            models: &HashMap<ModelId, Model>)
                            -> Result<(), CloudError> {
        match flow_id {
            NodePadId::MixerInput(mixer_id) => self.check_channel_exists_mixer(id, mixer_id, channels),
            NodePadId::MixerOutput(mixer_id) => self.check_channel_exists_mixer(id, mixer_id, channels),
            NodePadId::FixedInstanceInput(fixed_id) => self.check_channel_exists_fixed(id, fixed_id, channels, false, models),
            NodePadId::FixedInstanceOutput(fixed_id) => self.check_channel_exists_fixed(id, fixed_id, channels, true, models),
            NodePadId::DynamicInstanceInput(dynamic_id) => self.check_channel_exists_dynamic(id, dynamic_id, channels, false, models),
            NodePadId::DynamicInstanceOutput(dynamic_id) => self.check_channel_exists_dynamic(id, dynamic_id, channels, true, models),
            NodePadId::TrackOutput(track_id) => self.check_channel_exists_track(id, track_id, channels),
        }
    }

    fn check_channel_exists_mixer(&self, id: &NodeConnectionId, mixer_id: &MixerNodeId, channels: &ChannelMask) -> Result<(), CloudError> {
        let mixer =
            self.mixers
                .get(mixer_id)
                .ok_or_else(|| InternalInconsistency { message: format!("Connection {id} flow to mixer {mixer_id} does not exist"), })?;

        if !channels.is_subset_of(0..mixer.input_channels) {
            return Err(InternalInconsistency { message: format!("Connection {id} flow to mixer {mixer_id} has channels that do not exist"), });
        }

        Ok(())
    }

    fn check_channel_exists_fixed(&self,
                                  id: &NodeConnectionId,
                                  fixed_id: &FixedInstanceNodeId,
                                  channels: &ChannelMask,
                                  output: bool,
                                  models: &HashMap<ModelId, Model>)
                                  -> Result<(), CloudError> {
        let fixed = self.fixed
                        .get(fixed_id)
                        .ok_or_else(|| InternalInconsistency { message: format!("Connection {id} references fixed {fixed_id} which does not exist")})?;

        let model_id = fixed.instance_id.model_id();
        let model = models.get(&model_id).ok_or_else(|| {
            InternalInconsistency { message: format!("Connection {id} references fixed instance labelled {fixed_id} which references model {model_id} which does not exist")}
        })?;

        if !channels.is_subset_of(0..(if output { model.outputs.len() } else { model.inputs.len() })) {
            return Err(InternalInconsistency { message: format!("Connection {id} references fixed instance labelled {fixed_id} which has channels that do not exist")});
        }

        Ok(())
    }

    fn check_channel_exists_dynamic(&self,
                                    id: &NodeConnectionId,
                                    dynamic_id: &DynamicInstanceNodeId,
                                    channels: &ChannelMask,
                                    output: bool,
                                    models: &HashMap<ModelId, Model>)
                                    -> Result<(), CloudError> {
        let dynamic = self.dynamic.get(dynamic_id).ok_or_else(|| {
            InternalInconsistency{message: format!("Connection {id} references dynamic instance labelled {dynamic_id} which does not exist")}
        })?;

        let model_id = &dynamic.model_id;
        let model = models.get(&model_id).ok_or_else(|| {
            InternalInconsistency{ message: format!("Connection {id} references dynamic instance labelled {dynamic_id} which references model {model_id} which does not exist")}
        })?;

        if !channels.is_subset_of(0..(if output { model.outputs.len() } else { model.inputs.len() })) {
            return Err(InternalInconsistency{ message: format!("Connection {id} references dynamic instance labelled {dynamic_id} which has channels that do not exist")});
        }

        Ok(())
    }

    fn check_channel_exists_track(&self, id: &NodeConnectionId, track_id: &TrackNodeId, channels: &ChannelMask) -> Result<(), CloudError> {
        let track = self.tracks
                        .get(track_id)
                        .ok_or_else(|| InternalInconsistency { message: format!("Connection {id} references track {track_id} which does not exist")})?;

        if !channels.is_subset_of(0..track.channels.num_channels()) {
            return Err(InternalInconsistency{ message: format!("Connection {id} references track {track_id} which has channels that do not exist")});
        }

        Ok(())
    }
}

/// Task information
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Task {
    /// Domain executing the task
    pub domain_id:           DomainId,
    /// Reservation time range
    pub time:                TimeRange,
    /// Task specification
    pub spec:                TaskSpec,
    /// Security keys and associateds permissions
    pub security:            HashMap<SecureKey, TaskPermissions>,
    /// The pool of fixed isntances available to the task during its reserved time
    pub fixed_instance_pool: HashSet<FixedInstanceId>,
    /// Current version of the task, incremented by every change transaction
    pub version:             u64,
}

impl From<CreateTask> for Task {
    fn from(source: CreateTask) -> Self {
        let CreateTask { time,
                         domain,
                         tracks,
                         mixers,
                         dynamic,
                         fixed,
                         security,
                         connections,
                         fixed_instance_pool,
                         .. } = source;

        Self { domain_id: domain,
               time,
               security,
               fixed_instance_pool,
               version: 0,
               spec: TaskSpec { tracks,
                                mixers,
                                dynamic,
                                fixed,
                                connections } }
    }
}

/// Mixer node specification
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct MixerNode {
    /// Numvber of input channels on the mixer node
    pub input_channels:  usize,
    /// Number of output channels on the mixer node
    pub output_channels: usize,
}

/// Dynamic node specification
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct DynamicInstanceNode {
    /// The manufacturer and name of the processing software
    pub model_id:   ModelId,
    /// Parameter values
    pub parameters: InstanceParameters,
}

/// Fixed instance node specification
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct FixedInstanceNode {
    /// The manufacturer, name and instance identifier of the hardware device doing the processing
    pub instance_id: FixedInstanceId,
    /// parameters
    pub parameters:  InstanceParameters,
    /// Dry-wet percentage
    ///
    /// only applicable for instances with same number of inputs and outputs,
    /// having 1 or 2 channels.
    pub wet:         f64,
}

/// Connection between nodes in a task
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct NodeConnection {
    /// Source node pad
    pub from:          NodePadId,
    /// Destination node pad
    pub to:            NodePadId,
    /// Source channel mask
    pub from_channels: ChannelMask,
    /// Destination channel mask
    pub to_channels:   ChannelMask,
    /// Volume adjustment as a factor
    pub volume:        f64,
    /// Panning adjustment
    ///
    /// Zero is centered, -1 is fully left, 1 is fully right
    pub pan:           f64,
}

pub type InstanceParameters = serde_json::Value;
pub type InstanceReports = HashMap<ReportId, MultiChannelTimestampedValue>;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct ConnectionValues {
    pub volume: Option<f64>,
    pub pan:    Option<f64>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, IsVariant, Unwrap, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MixerChannels {
    Mono(usize),
    Stereo(usize),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, IsVariant, Unwrap, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ChannelMask {
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

impl ChannelMask {
    pub fn to_count_and_index(self) -> (usize, usize) {
        match self {
            Self::Mono(ch) => (1, ch),
            Self::Stereo(ch) => (2, ch),
        }
    }

    pub fn is_subset_of(self, range: Range<usize>) -> bool {
        match self {
            Self::Mono(ch) => range.contains(&ch),
            Self::Stereo(ch) => range.contains(&ch) && range.contains(&(ch + 1)),
        }
    }
}

/// An input or output pad of a node inside a task
#[derive(Clone, Debug, PartialEq, IsVariant, Unwrap, Hash, Eq, PartialOrd, Ord)]
pub enum NodePadId {
    /// Mixer node input pad
    MixerInput(MixerNodeId),
    /// Mixer node output pad
    MixerOutput(MixerNodeId),
    /// Fixed instance input pad
    FixedInstanceInput(FixedInstanceNodeId),
    /// Fixed instance output pad
    FixedInstanceOutput(FixedInstanceNodeId),
    /// Dynamic instance input pad
    DynamicInstanceInput(DynamicInstanceNodeId),
    /// Dynamic instance output pad
    DynamicInstanceOutput(DynamicInstanceNodeId),
    /// Track node output pad
    TrackOutput(TrackNodeId),
}

impl NodePadId {
    pub fn is_input(&self) -> bool {
        matches!(self,
                 NodePadId::MixerInput(_) | NodePadId::FixedInstanceInput(_) | NodePadId::DynamicInstanceInput(_))
    }

    pub fn is_output(&self) -> bool {
        matches!(self,
                 NodePadId::MixerOutput(_)
                 | NodePadId::FixedInstanceOutput(_)
                 | NodePadId::DynamicInstanceOutput(_)
                 | NodePadId::TrackOutput(_))
    }
}

impl std::fmt::Display for NodePadId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner_json = match serde_json::to_value(self).unwrap() {
            serde_json::Value::String(s) => s,
            _ => unreachable!(),
        };
        f.write_str(&inner_json)
    }
}

impl Serialize for NodePadId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&match self {
                                     NodePadId::MixerInput(mixer) => format!("mix:inp:{mixer}"),
                                     NodePadId::FixedInstanceInput(fixed) => format!("fix:inp:{fixed}"),
                                     NodePadId::DynamicInstanceInput(dynamic) => format!("dyn:inp:{dynamic}"),
                                     NodePadId::MixerOutput(mixer) => format!("mix:out:{mixer}"),
                                     NodePadId::FixedInstanceOutput(fixed) => format!("fix:out:{fixed}"),
                                     NodePadId::DynamicInstanceOutput(dynamic) => format!("dyn:out:{dynamic}"),
                                     NodePadId::TrackOutput(track) => format!("trk:out:{track}"),
                                 })
    }
}

impl<'de> Deserialize<'de> for NodePadId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let err = |msg| serde::de::Error::custom(msg);
        let string = String::deserialize(deserializer)?;
        let sep_pos = string.find(':').ok_or_else(|| err("expected separator ':'"))?;
        let sep_pos2 = string[(sep_pos + 1)..].find(':').ok_or_else(|| err("expected separator ':'"))?;
        let rest = string[(sep_pos + sep_pos2 + 2)..].to_owned();

        Ok(match (&string[..sep_pos], &string[(sep_pos + 1)..(sep_pos + sep_pos2 + 1)]) {
            ("mix", "inp") => Self::MixerInput(MixerNodeId::new(rest)),
            ("mix", "out") => Self::MixerOutput(MixerNodeId::new(rest)),
            ("fix", "inp") => Self::FixedInstanceInput(FixedInstanceNodeId::new(rest)),
            ("fix", "out") => Self::FixedInstanceOutput(FixedInstanceNodeId::new(rest)),
            ("dyn", "inp") => Self::DynamicInstanceInput(DynamicInstanceNodeId::new(rest)),
            ("dyn", "out") => Self::DynamicInstanceOutput(DynamicInstanceNodeId::new(rest)),
            ("trk", "out") => Self::TrackOutput(TrackNodeId::new(rest)),
            (a, b) => return Err(err(&format!("unrecognized NodePadId variant: '{a}', '{b}'"))),
        })
    }
}

impl FromStr for NodePadId {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

/// Track node specification
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct TrackNode {
    /// Number of channels
    pub channels: MediaChannels,
    /// Media items present on the track
    pub media:    HashMap<TrackMediaId, TrackMedia>,
}

/// Channel count for media items and track nodes
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MediaChannels {
    /// Single channel
    Mono,
    /// Two channels - left and right
    Stereo,
}

impl MediaChannels {
    pub fn num_channels(&self) -> usize {
        match self {
            MediaChannels::Mono => 1,
            MediaChannels::Stereo => 2,
        }
    }
}

/// Media item specification
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct TrackMedia {
    /// Number of channels
    pub channels:         MediaChannels,
    /// Media format
    pub format:           TrackMediaFormat,
    /// Subset of media that is used
    pub media_segment:    TimeSegment,
    /// Where to place the media in the task timeline
    pub timeline_segment: TimeSegment,
    /// Source media object id
    pub object_id:        MediaObjectId,
}

impl TrackMedia {
    pub fn update(&mut self, update: UpdateTaskTrackMedia) {
        let UpdateTaskTrackMedia { channels,
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct UpdateTaskTrackMedia {
    pub channels:         Option<MediaChannels>,
    pub media_segment:    Option<TimeSegment>,
    pub timeline_segment: Option<TimeSegment>,
    pub object_id:        Option<MediaObjectId>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub enum TrackMediaFormat {
    #[serde(rename = "wave")]
    Wave,
    #[serde(rename = "mp3")]
    Mp3,
    #[serde(rename = "flac")]
    Flac,
    #[serde(rename = "wavpack")]
    WavPack,
}

impl Display for TrackMediaFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match serde_json::to_value(self).unwrap() {
            Value::String(s) => s,
            _ => unreachable!(),
        };
        f.write_str(&s)
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct TimeSegment {
    pub start:  f64,
    pub length: f64,
}

impl TimeSegment {
    pub fn end(&self) -> f64 {
        self.start + self.length
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct TaskPermissions {
    pub structure:  bool,
    pub media:      bool,
    pub parameters: bool,
    pub transport:  bool,
    pub audio:      bool,
}

impl TaskPermissions {
    pub fn full() -> Self {
        TaskPermissions { structure:  true,
                          media:      true,
                          parameters: true,
                          transport:  true,
                          audio:      true, }
    }
}

json_schema_new_type!(NodePadId);
