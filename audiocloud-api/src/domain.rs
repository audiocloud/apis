//! API definitions for communicating with the domain
//!
//! The domain will communicate with either apps that connect directly to them
//! or with the cloud.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::change::SessionState;
use crate::error::SerializableResult;
use crate::newtypes::{AppMediaObjectId, AppSessionId, SecureKey};
use crate::session::SessionSecurity;
use crate::{
    app::SessionPacket,
    change::{DesiredSessionPlayState, ModifySessionSpec},
    cloud::apps::{CreateSession, SessionSpec},
    media::DownloadMedia,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DomainSessionCommand {
    Create {
        app_session_id: AppSessionId,
        create:         CreateSession,
    },
    SetSpec {
        app_session_id: AppSessionId,
        version:        u64,
        spec:           SessionSpec,
    },
    SetSecurity {
        app_session_id: AppSessionId,
        version:        u64,
        security:       HashMap<SecureKey, SessionSecurity>,
    },
    Modify {
        app_session_id: AppSessionId,
        version:        u64,
        modifications:  Vec<ModifySessionSpec>,
    },
    SetDesiredPlayState {
        app_session_id:     AppSessionId,
        version:            u64,
        desired_play_state: DesiredSessionPlayState,
    },
    Delete {
        app_session_id: AppSessionId,
    },
}

impl DomainSessionCommand {
    pub fn get_session_id(&self) -> &AppSessionId {
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
    Packet(AppSessionId, SessionPacket),
    Spec(AppSessionId, SessionSpec),
    State(AppSessionId, SessionState),
    SessionError(AppSessionId, String),
    Response(String, SerializableResult<()>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum WebSocketCommand {
    Login(AppSessionId, SecureKey),
    Logout(AppSessionId),
    Session(DomainSessionCommand),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebSocketCommandEnvelope {
    pub request_id: String,
    pub command:    WebSocketCommand,
}
