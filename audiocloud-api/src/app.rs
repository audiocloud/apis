//! API definitions for communicating with the apps
use std::collections::{HashMap, HashSet};

use chrono::Utc;

use crate::audio_engine::CompressedAudio;
use crate::change::{DesiredSessionPlayState, SessionPlayState};
use crate::instance::InstancePlayState;
use crate::instance::InstancePowerState;
use crate::model::MultiChannelValue;
use crate::newtypes::{DynamicId, FixedId, FixedInstanceId, MediaObjectId, MixerId, ReportId, TrackId};
use crate::session::InstanceReports;
use crate::time::{Timestamp, Timestamped};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionPacket {
    pub created_at:            Timestamp,
    pub fixed:                 HashMap<FixedId, FixedInstancePacket>,
    pub dynamic:               HashMap<DynamicId, DynamicInstancePacket>,
    pub mixers:                HashMap<MixerId, MixerPacket>,
    pub tracks:                HashMap<TrackId, TrackPacket>,
    pub waiting_for_instances: HashSet<FixedInstanceId>,
    pub waiting_for_media:     HashSet<MediaObjectId>,
    pub compressed_audio:      Vec<CompressedAudio>,
    pub desired_play_state:    DesiredSessionPlayState,
    pub play_state:            SessionPlayState,
    pub audio_engine_ready:    bool,
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
               desired_play_state:    DesiredSessionPlayState::Stopped,
               play_state:            SessionPlayState::Stopped,
               audio_engine_ready:    false, }
    }
}

impl SessionPacket {
    pub fn push_fixed_instance_reports(&mut self, instance: FixedId, reports: InstanceReports) {
        let fixed = self.fixed.entry(instance).or_default();

        for (report_id, value) in reports {
            fixed.instance_metering
                 .entry(report_id)
                 .or_default()
                 .push());
        }
    }

    pub fn add_waiting_instance(&mut self, instance_id: &FixedInstanceId) {
        if !self.waiting_for_instances.contains(instance_id) {
            self.waiting_for_instances.insert(instance_id.clone());
        }
    }

    pub fn add_waiting_media(&mut self, media_id: &MediaObjectId) {
        if !self.waiting_for_media.contains(media_id) {
            self.waiting_for_media.insert(media_id.clone());
        }
    }

    pub fn push_fixed_error(&mut self, instance: FixedId, error: String) {
        self.fixed.entry(instance).or_default().errors.push(error);
    }

    pub fn push_fixed_input_metering(&mut self, fixed_id: &FixedId, input: MultiChannelValue) {
        let fixed = self.fixed.entry(fixed_id.clone()).or_default();

        fixed.input_metering.push(DiffStamped::from((self.created_at, input)));
    }

    pub fn push_fixed_output_metering(&mut self, fixed_id: &FixedId, output: MultiChannelValue) {
        let fixed = self.fixed.entry(fixed_id.clone()).or_default();

        fixed.output_metering.push(DiffStamped::from((self.created_at, output)));
    }

    pub fn push_dynamic_input_metering(&mut self, dynamic_id: &DynamicId, input: MultiChannelValue) {
        let dynamic = self.dynamic.entry(dynamic_id.clone()).or_default();

        dynamic.input_metering.push(DiffStamped::from((self.created_at, input)));
    }

    pub fn push_dynamic_output_metering(&mut self, dynamic_id: &DynamicId, output: MultiChannelValue) {
        let dynamic = self.dynamic.entry(dynamic_id.clone()).or_default();

        dynamic.output_metering.push(DiffStamped::from((self.created_at, output)));
    }

    pub fn push_mixer_output_metering(&mut self, mixer_id: &MixerId, output: MultiChannelValue) {
        let mixer = self.mixers.entry(mixer_id.clone()).or_default();

        mixer.output_metering.push(DiffStamped::from((self.created_at, output)));
    }

    pub fn push_track_output_metering(&mut self, track_id: &TrackId, output: MultiChannelValue) {
        let track = self.tracks.entry(track_id.clone()).or_default();

        track.output_metering.push(DiffStamped::from((self.created_at, output)));
    }

    pub fn push_audio_packets(&mut self, compressed_audio: CompressedAudio) {
        self.compressed_audio.push(compressed_audio);
    }
}

/// Difference stamped in milliseconds since a common epoch, in order to pack most efficiently
/// The epoch in InstancePacket is the created_at field of SessionPacket
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiffStamped<T>(usize, T);

impl<T> From<(Timestamp, T)> for DiffStamped<T> {
    fn from(value: (Timestamp, T)) -> Self {
        let (timestamp, value) = value;
        let diff = Utc::now() - timestamp;
        Self(diff.num_milliseconds() as usize, value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FixedInstancePacket {
    pub errors:            Vec<String>,
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
    pub output_metering: Vec<DiffStamped<MultiChannelValue>>,
}
