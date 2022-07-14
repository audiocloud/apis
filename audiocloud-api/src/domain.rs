//! API definitions for communicating with the domain
//!
//! The domain will communicate with either apps that connect directly to them
//! or with the cloud.

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::newtypes::SecureKey;
use crate::session::SessionSecurity;
use crate::{
    app::SessionPacket,
    change::{DesiredSessionPlayState, ModifySessionSpec},
    cloud::apps::{CreateSession, SessionSpec},
    media::{DownloadMedia, MediaDownloadState},
    newtypes::{AppId, MediaObjectId, SessionId},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DomainSessionCommand {
    Create {
        create: CreateSession,
    },
    SetSpec {
        app_id:     AppId,
        session_id: SessionId,
        spec:       SessionSpec,
    },
    SetSecurity {
        app_id:     AppId,
        session_id: SessionId,
        security:   HashMap<SecureKey, SessionSecurity>,
    },
    Modify {
        app_id:        AppId,
        session_id:    SessionId,
        modifications: Vec<ModifySessionSpec>,
    },
    SetDesiredPlayState {
        app_id:             AppId,
        session_id:         SessionId,
        desired_play_state: DesiredSessionPlayState,
    },
    Delete {
        app_id:     AppId,
        session_id: SessionId,
    },
}

impl DomainSessionCommand {
    pub fn get_session_id(&self) -> &SessionId {
        match self {
            DomainSessionCommand::Create { create } => &create.id,
            DomainSessionCommand::SetSpec { session_id, .. } => session_id,
            DomainSessionCommand::SetSecurity { session_id, .. } => session_id,
            DomainSessionCommand::Modify { session_id, .. } => session_id,
            DomainSessionCommand::SetDesiredPlayState { session_id, .. } => session_id,
            DomainSessionCommand::Delete { session_id, .. } => session_id,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DomainMediaCommand {
    Download {
        app_id:   AppId,
        media_id: MediaObjectId,
        download: DownloadMedia,
    },
    Delete {
        app_id:   AppId,
        media_id: MediaObjectId,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum WebSocketEvent {
    Packet(SessionId, Arc<SessionPacket>),
    Download(MediaObjectId, MediaDownloadState),
}
