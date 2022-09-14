//! API definitions for communicating with the apps
use std::collections::{HashMap, HashSet};

use chrono::Utc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::audio_engine::CompressedAudio;
use crate::common::change::{DesiredTaskPlayState, TaskPlayState};
use crate::common::media::{PlayId, RenderId};
use crate::common::task::InstanceReports;
use crate::common::time::{Timestamp, Timestamped};
use crate::common::{
    AppMediaObjectId, DynamicInstanceNodeId, FixedInstanceId, FixedInstanceNodeId, InstancePlayState, InstancePowerState, MixerNodeId,
    MultiChannelValue, ReportId, TrackNodeId,
};
use crate::{AppTaskId, DestinationPadId, SourcePadId};

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct TaskStreamingPacket {
    pub task_id:               AppTaskId,
    pub play_id:               PlayId,
    pub serial:                u64,
    pub created_at:            Timestamp,
    pub fixed:                 HashMap<FixedInstanceNodeId, FixedInstancePacket>,
    pub dynamic:               HashMap<DynamicInstanceNodeId, DynamicInstancePacket>,
    pub mixers:                HashMap<MixerNodeId, MixerPacket>,
    pub tracks:                HashMap<TrackNodeId, TrackPacket>,
    pub waiting_for_instances: HashSet<FixedInstanceId>,
    pub waiting_for_media:     HashSet<AppMediaObjectId>,
    pub compressed_audio:      Vec<CompressedAudio>,
    pub desired_play_state:    DesiredTaskPlayState,
    pub play_state:            TaskPlayState,
    pub errors:                Vec<Timestamped<SessionPacketError>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct StreamStats {
    pub id:    AppTaskId,
    pub state: TaskPlayState,
    pub play:  PlayId,
    pub low:   u64,
    pub high:  u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SessionPacketError {
    Playing(PlayId, String),
    Rendering(RenderId, String),
    General(String),
}

impl TaskStreamingPacket {
    pub fn push_fixed_instance_reports(&mut self, instance: FixedInstanceNodeId, reports: InstanceReports) {
        let fixed = self.fixed.entry(instance).or_default();

        // for (report_id, value) in reports {
        //     fixed.instance_metering
        //          .entry(report_id)
        //          .or_default()
        //          .push();
        // }
    }

    pub fn push_source_peak_meters(&mut self, peak_meters: HashMap<SourcePadId, MultiChannelValue>) {
        for (flow_id, value) in peak_meters {
            match flow_id {
                SourcePadId::MixerOutput(mixer_id) => {
                    self.mixers
                        .entry(mixer_id)
                        .or_default()
                        .output_metering
                        .push(DiffStamped::new(self.created_at, value));
                }
                SourcePadId::FixedInstanceOutput(fixed_id) => {
                    self.fixed
                        .entry(fixed_id)
                        .or_default()
                        .output_metering
                        .push(DiffStamped::new(self.created_at, value));
                }
                SourcePadId::DynamicInstanceOutput(dynamic_id) => {
                    self.dynamic
                        .entry(dynamic_id)
                        .or_default()
                        .output_metering
                        .push(DiffStamped::new(self.created_at, value));
                }
                SourcePadId::TrackOutput(track_id) => {
                    self.tracks
                        .entry(track_id)
                        .or_default()
                        .output_metering
                        .push(DiffStamped::new(self.created_at, value));
                }
            }
        }
    }

    pub fn push_destination_peak_meters(&mut self, peak_meters: HashMap<DestinationPadId, MultiChannelValue>) {
        for (flow_id, value) in peak_meters {
            match flow_id {
                DestinationPadId::MixerInput(mixer_id) => {
                    self.mixers
                        .entry(mixer_id)
                        .or_default()
                        .input_metering
                        .push(DiffStamped::new(self.created_at, value));
                }
                DestinationPadId::FixedInstanceInput(fixed_id) => {
                    self.fixed
                        .entry(fixed_id)
                        .or_default()
                        .input_metering
                        .push(DiffStamped::new(self.created_at, value));
                }
                DestinationPadId::DynamicInstanceInput(dynamic_id) => {
                    self.dynamic
                        .entry(dynamic_id)
                        .or_default()
                        .input_metering
                        .push(DiffStamped::new(self.created_at, value));
                }
            }
        }
    }

    pub fn add_play_error(&mut self, play_id: PlayId, error: String) {
        self.errors.push(Timestamped::new(SessionPacketError::Playing(play_id, error)));
    }

    pub fn add_render_error(&mut self, render_id: RenderId, error: String) {
        self.errors.push(Timestamped::new(SessionPacketError::Rendering(render_id, error)));
    }

    pub fn push_fixed_error(&mut self, instance: FixedInstanceNodeId, error: String) {
        self.fixed.entry(instance).or_default().errors.push(Timestamped::new(error));
    }

    pub fn push_audio_packets(&mut self, compressed_audio: CompressedAudio) {
        self.compressed_audio.push(compressed_audio);
    }
}

/// Difference stamped in milliseconds since a common epoch, in order to pack most efficiently
/// The epoch in InstancePacket is the created_at field of SessionPacket
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct DiffStamped<T>(usize, T);

impl<T> DiffStamped<T> {
    pub fn new(timestamp: Timestamp, value: T) -> Self {
        (timestamp, value).into()
    }
}

impl<T> From<(Timestamp, T)> for DiffStamped<T> {
    fn from(value: (Timestamp, T)) -> Self {
        let (timestamp, value) = value;
        let diff = Utc::now() - timestamp;
        Self(diff.num_milliseconds() as usize, value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, JsonSchema)]
pub struct FixedInstancePacket {
    pub errors:            Vec<Timestamped<String>>,
    pub instance_metering: HashMap<ReportId, Vec<DiffStamped<MultiChannelValue>>>,
    pub input_metering:    Vec<DiffStamped<MultiChannelValue>>,
    pub output_metering:   Vec<DiffStamped<MultiChannelValue>>,
    pub media_pos:         Option<f64>,
    pub power:             Option<Timestamped<InstancePowerState>>,
    pub media:             Option<Timestamped<InstancePlayState>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, JsonSchema)]
pub struct DynamicInstancePacket {
    pub instance_metering: HashMap<ReportId, Vec<DiffStamped<MultiChannelValue>>>,
    pub input_metering:    Vec<DiffStamped<MultiChannelValue>>,
    pub output_metering:   Vec<DiffStamped<MultiChannelValue>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, JsonSchema)]
pub struct TrackPacket {
    pub output_metering: Vec<DiffStamped<MultiChannelValue>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, JsonSchema)]
pub struct MixerPacket {
    pub input_metering:  Vec<DiffStamped<MultiChannelValue>>,
    pub output_metering: Vec<DiffStamped<MultiChannelValue>>,
}

/// Load packet data
///
/// For each PlayId, on a task, a stream is kept in memory with a history of packets, by ascending
/// serial number. For a sane amount of time, the packets may be requested by the clients. If a
/// packet is not yet models (but it is expected they will be, in the future) the request will
/// block (wait) for `Timeout` milliseconds before giving up and returning 408.
#[utoipa::path(
  get,
  path = "/v1/stream/{app_id}/{task_id}/{play_id}/packet/{serial}",
  responses(
    (status = 200, description = "Success", body = TaskStreamingPacket),
    (status = 401, description = "Not authorized", body = DomainError),
    (status = 404, description = "App, task or stream not found", body = DomainError),
    (status = 408, description = "Timed out waiting for packet", body = DomainError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id"),
    ("play_id" = PlayId, Path, description = "Play id"),
    ("serial" = u64, Path, description = "Packet serial number"),
    ("Timeout" = u64, Header, description = "Milliseconds to wait for the packet to be ready")
  ))]
pub(crate) fn stream_packets() {}

/// Get stream statistics
///
/// Get statistics about cached packets available in the stream.
#[utoipa::path(
  get,
  path = "/v1/stream/{app_id}/{task_id}/{play_id}",
  responses(
    (status = 200, description = "Success", body = StreamStats),
    (status = 401, description = "Not authorized", body = DomainError),
    (status = 404, description = "Not found", body = DomainError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id"),
    ("play_id" = PlayId, Path, description = "Play id")
  ))]
pub(crate) fn stats() {}
