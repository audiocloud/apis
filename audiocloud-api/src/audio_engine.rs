//! The API to the audio engine (from the domain side)

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::change::{ModifySessionSpec, PlayId, PlaySession, RenderId, RenderSession};
use crate::cloud::apps::SessionSpec;
use crate::model::{MultiChannelTimestampedValue, MultiChannelValue};
use crate::newtypes::{AppSessionId, DynamicId, ParameterId, ReportId};
use crate::session::SessionFlowId;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AudioEngineCommand {
    SetSpec {
        session_id: AppSessionId,
        spec:       SessionSpec,
    },
    ModifySpec {
        session_id:  AppSessionId,
        transaction: Vec<ModifySessionSpec>,
    },
    SetDynamicParameters {
        session_id: AppSessionId,
        dynamic_id: DynamicId,
        parameters: HashMap<ParameterId, MultiChannelValue>,
    },
    Render {
        session_id: AppSessionId,
        render:     RenderSession,
    },
    Play {
        session_id: AppSessionId,
        play:       PlaySession,
    },
    Stop {
        session_id: AppSessionId,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AudioEngineEvent {
    Loaded,
    Stopped {
        session_id: AppSessionId,
    },
    Playing {
        session_id:      AppSessionId,
        playing:         PlaySession,
        audio:           CompressedAudio,
        peak_meters:     HashMap<SessionFlowId, MultiChannelValue>,
        dynamic_reports: HashMap<DynamicId, HashMap<ReportId, MultiChannelTimestampedValue>>,
    },
    PlayingFailed {
        session_id: AppSessionId,
        play_id:    PlayId,
        error:      String,
    },
    Rendering {
        session_id: AppSessionId,
        rendering:  RenderSession,
    },
    RenderingFinished {
        session_id: AppSessionId,
        render_id:  RenderId,
        path:       String,
    },
    RenderingFailed {
        session_id: AppSessionId,
        render_id:  RenderId,
        reason:     String,
    },
    Exit {
        code: i32,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CompressedAudio {
    pub play_id:      PlayId,
    pub timeline_pos: f64,
    pub stream_pos:   u64,
    #[serde(with = "serde_bytes")]
    pub buffer:       Vec<u8>,
    pub last:         bool,
}

#[derive(Debug, Error, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AudioEngineError {
    #[error("Track {0} not found")]
    TrackNotFound(usize),

    #[error("Item {0} on track {1} not found")]
    ItemNotFound(usize, usize),

    #[error("Internal sound engine error: {0}")]
    InternalError(String),

    #[error("Remote call failed: {0}")]
    RPC(String),
}
