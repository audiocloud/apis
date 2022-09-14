use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::audio_engine::CompressedAudio;
use crate::common::media::{PlayId, RenderId};
use crate::{AppTaskId, DestinationPadId, DynamicInstanceNodeId, MultiChannelTimestampedValue, MultiChannelValue, ReportId, SourcePadId};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AudioEngineEvent {
    Stopped {
        task_id: AppTaskId,
    },
    Playing {
        task_id:            AppTaskId,
        play_id:            PlayId,
        audio:              CompressedAudio,
        input_peak_meters:  HashMap<DestinationPadId, MultiChannelValue>,
        output_peak_meters: HashMap<SourcePadId, MultiChannelValue>,
        dynamic_reports:    HashMap<DynamicInstanceNodeId, HashMap<ReportId, MultiChannelTimestampedValue>>,
    },
    PlayingFailed {
        task_id: AppTaskId,
        play_id: PlayId,
        error:   String,
    },
    Rendering {
        task_id:    AppTaskId,
        render_id:  RenderId,
        completion: f64,
    },
    RenderingFinished {
        task_id:   AppTaskId,
        render_id: RenderId,
        path:      String,
    },
    RenderingFailed {
        task_id:   AppTaskId,
        render_id: RenderId,
        reason:    String,
    },
    Error {
        task_id: AppTaskId,
        error:   String,
    },
}

impl AudioEngineEvent {
    pub fn task_id(&self) -> &AppTaskId {
        match self {
            AudioEngineEvent::Stopped { task_id } => task_id,
            AudioEngineEvent::Playing { task_id, .. } => task_id,
            AudioEngineEvent::PlayingFailed { task_id, .. } => task_id,
            AudioEngineEvent::Rendering { task_id, .. } => task_id,
            AudioEngineEvent::RenderingFinished { task_id, .. } => task_id,
            AudioEngineEvent::RenderingFailed { task_id, .. } => task_id,
            AudioEngineEvent::Error { task_id, .. } => task_id,
        }
    }
}
