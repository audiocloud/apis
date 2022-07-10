//! The API to the audio engine (from the domain side)

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::change::{PlayId, PlaySegment, PlaySession, RenderId, RenderSession};
use crate::codec::MsgPack;
use crate::model::MultiChannelValue;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AudioEngineCommand {
  SetTrackStateChunk {
    track_id: Uuid,
    chunk:    String,
  },
  SetItemStateChunk {
    track_id: Uuid,
    item_id:  Uuid,
    chunk:    String,
  },
  SetTrackValues {
    track_id:    Uuid,
    volume:      Option<f64>,
    pan:         Option<f64>,
    master_send: Option<bool>,
  },
  SetReceiveValues {
    track_id:         Uuid,
    receive_track_id: Uuid,
    volume:           Option<f64>,
    pan:              Option<f64>,
  },
  SetFXValues {
    track_id: Uuid,
    fx_id:    Uuid,
    values:   HashMap<u32, f64>,
  },
  SetFXStateValues {
    track_id: Uuid,
    fx_id:    Uuid,
    enabled:  Option<bool>,
    dry_wet:  Option<f64>,
  },
  SetMaster {
    track_id: Uuid,
  },
  DeleteTrack {
    track_id: Uuid,
  },
  Play(PlaySession),
  SetPlaySegment(PlaySegment),
  Stop {},
  Render(RenderSession),
  Exit {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AudioEngineEvent {
  Loaded,
  Stopped,
  Playing { playing: PlaySession, audio: CompressedAudio },
  Rendering { rendering: RenderSession },
  RenderingFinished { render_id: RenderId, path: String },
  Meters { peak_meters: HashMap<Uuid, MultiChannelValue> },
  Exit { code: i32 },
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
