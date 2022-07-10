//! Types used to communicate with the driver

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::change::{PlayId, RenderId};
use crate::instance::{DesiredInstancePlayState, InstancePlayState};
use crate::model::MultiChannelValue;
use crate::newtypes::{ParameterId, ReportId};

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum InstanceDriverCommand {
  CheckConnection,
  Stop,
  Play { play_id: PlayId },
  Render { length: f64, render_id: RenderId },
  Rewind { to: f64 },
  SetParameters(HashMap<ParameterId, MultiChannelValue>),
}

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug, Error)]
#[serde(rename_all = "snake_case")]
pub enum InstanceDriverError {
  #[error("Parameter {parameter} does not exist")]
  ParameterDoesNotExist { parameter: String },

  #[error("Media is not present, can't play or rewind")]
  MediaNotPresent,

  #[error("Driver can't guarantee that playback won't be interrupted")]
  NotInterruptable,

  #[error("Remote call failed: {error}")]
  RPC { error: String },
}

#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum InstanceDriverEvent {
  /// Sent when the driver has started
  Started,

  /// If an I/O error happened during communication with device
  IOError { error: String },

  /// Driver lost connection to the hardware
  ConnectionLost,

  /// Driver connected to the hardware
  Connected,

  /// Received metering updates from the hardware
  Metering { meters: HashMap<ReportId, MultiChannelValue> },

  /// Playing; media current position reported
  PlayState {
    desired: DesiredInstancePlayState,
    current: InstancePlayState,
    media:   Option<f64>,
  },
}
