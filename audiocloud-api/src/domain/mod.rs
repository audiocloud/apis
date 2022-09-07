//! API definitions for communicating with the domain
//!
//! The domain will communicate with either apps that connect directly to them
//! or with the cloud.

use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::app::SessionPacket;
use crate::cloud::tasks::CreateTask;
use crate::common::change::{DesiredTaskPlayState, ModifyTaskSpec};
use crate::common::change::SessionState;
use crate::common::error::SerializableResult;
use crate::common::task::TaskSecurity;
use crate::common::task::TaskSpec;
use crate::newtypes::{AppTaskId, SecureKey};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DomainSessionCommand {
    Create {
        app_session_id: AppTaskId,
        create:         CreateTask,
    },
    SetSpec {
        app_session_id: AppTaskId,
        version:        u64,
        spec:           TaskSpec,
    },
    SetSecurity {
        app_session_id: AppTaskId,
        version:        u64,
        security:       HashMap<SecureKey, TaskSecurity>,
    },
    Modify {
        app_session_id: AppTaskId,
        version:        u64,
        modifications:  Vec<ModifyTaskSpec>,
    },
    SetDesiredPlayState {
        app_session_id:     AppTaskId,
        version:            u64,
        desired_play_state: DesiredTaskPlayState,
    },
    Delete {
        app_session_id: AppTaskId,
    },
}

impl DomainSessionCommand {
    pub fn get_session_id(&self) -> &AppTaskId {
        match self {
            DomainSessionCommand::Create { app_session_id, .. } => app_session_id,
            DomainSessionCommand::SetSpec { app_session_id, .. } => app_session_id,
            DomainSessionCommand::SetSecurity { app_session_id, .. } => app_session_id,
            DomainSessionCommand::Modify { app_session_id, .. } => app_session_id,
            DomainSessionCommand::SetDesiredPlayState { app_session_id, .. } => app_session_id,
            DomainSessionCommand::Delete { app_session_id, .. } => app_session_id,
        }
    }

    pub fn get_kind(&self) -> &'static str {
        match self {
            DomainSessionCommand::Create { .. } => "create",
            DomainSessionCommand::SetSpec { .. } => "set_spec",
            DomainSessionCommand::SetSecurity { .. } => "set_security",
            DomainSessionCommand::Modify { .. } => "modify",
            DomainSessionCommand::SetDesiredPlayState { .. } => "set_desired_play_state",
            DomainSessionCommand::Delete { .. } => "delete",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum WebSocketEvent {
    Packet(AppTaskId, SessionPacket),
    Spec(AppTaskId, TaskSpec),
    State(AppTaskId, SessionState),
    SessionError(AppTaskId, String),
    Response(String, SerializableResult<()>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum WebSocketCommand {
    Login(AppTaskId, SecureKey),
    Logout(AppTaskId),
    Session(DomainSessionCommand),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebSocketCommandEnvelope {
    pub request_id: String,
    pub command:    WebSocketCommand,
}
