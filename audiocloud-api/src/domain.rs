//! API definitions for communicating with the domain
//!
//! The domain will communicate with either apps that connect directly to them
//! or with the cloud.

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::app::SessionPacket;
use crate::change::{DesiredSessionPlayState, ModifySession};
use crate::cloud::apps::CreateSession;
use crate::media::DomainMediaCommand;
use crate::media::MediaDownloadState;
use crate::newtypes::{FixedInstanceId, MediaObjectId, SessionId};
use crate::time::Timestamp;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DomainCommand {
  Session(SessionId, DomainSessionCommand),
  Instance(FixedInstanceId, DomainInstanceCommand),
  Media(MediaObjectId, DomainMediaCommand),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DomainSessionCommand {
  CreateOrReplace(CreateSession),
  Modify(Vec<ModifySession>),
  SetDesiredPlayState(DesiredSessionPlayState),
  Delete,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DomainInstanceCommand {
  Stop,
  Play { until: Timestamp },
  Rewind { to: f64 },
  PowerOff,
  PowerOn,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum WebSocketEvent {
  Packet(SessionId, Arc<SessionPacket>),
  Download(MediaObjectId, MediaDownloadState),
}
