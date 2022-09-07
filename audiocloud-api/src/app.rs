//! API definitions for communicating with the apps
use std::collections::{HashMap, HashSet};

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::audio_engine::CompressedAudio;
use crate::common::change::{DesiredTaskPlayState, TaskPlayState};
use crate::common::task::{InstanceReports, NodePadId};
use crate::common::time::{Timestamp, Timestamped};
use crate::common::{
    AppMediaObjectId, DynamicInstanceNodeId, FixedInstanceId, FixedInstanceNodeId, InstancePlayState, InstancePowerState, MixerNodeId,
    MultiChannelValue, ReportId, TrackNodeId,
};
use crate::common::media::{PlayId, RenderId};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionPacket {
    pub created_at:            Timestamp,
    pub fixed:                 HashMap<FixedInstanceNodeId, FixedInstancePacket>,
    pub dynamic:               HashMap<DynamicInstanceNodeId, DynamicInstancePacket>,
    pub mixers:                HashMap<MixerNodeId, MixerPacket>,
    pub tracks:                HashMap<TrackNodeId, TrackPacket>,
    pub waiting_for_instances: HashSet<FixedInstanceId>,
    pub waiting_for_media:     HashSet<AppMediaObjectId>,
    pub compressed_audio:      Vec<CompressedAudio>,
    pub desired_play_state: DesiredTaskPlayState,
    pub play_state: TaskPlayState,
    pub errors:                Vec<Timestamped<SessionPacketError>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SessionPacketError {
    Playing(PlayId, String),
    Rendering(RenderId, String),
    General(String),
}

impl Default for SessionPacket {
    fn default() -> Self {
        Self { created_at:            Utc::now(),
               fixed:                 Default::default(),
               dynamic:               Default::default(),
               mixers:                Default::default(),
               tracks:                Default::default(),
               waiting_for_instances: Default::default(),
               waiting_for_media:     Default::default(),
               compressed_audio:      Default::default(),
               desired_play_state:    DesiredTaskPlayState::Stopped,
               play_state:            TaskPlayState::Stopped,
               errors:                vec![], }
    }
}

impl SessionPacket {
    pub fn push_fixed_instance_reports(&mut self, instance: FixedInstanceNodeId, reports: InstanceReports) {
        let fixed = self.fixed.entry(instance).or_default();

        // for (report_id, value) in reports {
        //     fixed.instance_metering
        //          .entry(report_id)
        //          .or_default()
        //          .push();
        // }
    }

    pub fn push_peak_meters(&mut self, peak_meters: HashMap<NodePadId, MultiChannelValue>) {
        for (flow_id, value) in peak_meters {
            match flow_id {
                NodePadId::MixerInput(mixer_id) => {
                    self.mixers
                        .entry(mixer_id)
                        .or_default()
                        .input_metering
                        .push(DiffStamped::new(self.created_at, value));
                }
                NodePadId::MixerOutput(mixer_id) => {
                    self.mixers
                        .entry(mixer_id)
                        .or_default()
                        .output_metering
                        .push(DiffStamped::new(self.created_at, value));
                }
                NodePadId::FixedInstanceInput(fixed_id) => {
                    self.fixed
                        .entry(fixed_id)
                        .or_default()
                        .input_metering
                        .push(DiffStamped::new(self.created_at, value));
                }
                NodePadId::FixedInstanceOutput(fixed_id) => {
                    self.fixed
                        .entry(fixed_id)
                        .or_default()
                        .output_metering
                        .push(DiffStamped::new(self.created_at, value));
                }
                NodePadId::DynamicInstanceInput(dynamic_id) => {
                    self.dynamic
                        .entry(dynamic_id)
                        .or_default()
                        .input_metering
                        .push(DiffStamped::new(self.created_at, value));
                }
                NodePadId::DynamicInstanceOutput(dynamic_id) => {
                    self.dynamic
                        .entry(dynamic_id)
                        .or_default()
                        .output_metering
                        .push(DiffStamped::new(self.created_at, value));
                }
                NodePadId::TrackOutput(track_id) => {
                    self.tracks
                        .entry(track_id)
                        .or_default()
                        .output_metering
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
#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FixedInstancePacket {
    pub errors:            Vec<Timestamped<String>>,
    pub instance_metering: HashMap<ReportId, Vec<DiffStamped<MultiChannelValue>>>,
    pub input_metering:    Vec<DiffStamped<MultiChannelValue>>,
    pub output_metering:   Vec<DiffStamped<MultiChannelValue>>,
    pub media_pos:         Option<f64>,
    pub power:             Option<Timestamped<InstancePowerState>>,
    pub media:             Option<Timestamped<InstancePlayState>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DynamicInstancePacket {
    pub instance_metering: HashMap<ReportId, Vec<DiffStamped<MultiChannelValue>>>,
    pub input_metering:    Vec<DiffStamped<MultiChannelValue>>,
    pub output_metering:   Vec<DiffStamped<MultiChannelValue>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TrackPacket {
    pub output_metering: Vec<DiffStamped<MultiChannelValue>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MixerPacket {
    pub input_metering:  Vec<DiffStamped<MultiChannelValue>>,
    pub output_metering: Vec<DiffStamped<MultiChannelValue>>,
}
