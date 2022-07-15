//! API definitions for communicating with the domain
//!
//! The domain will communicate with either apps that connect directly to them
//! or with the cloud.

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::newtypes::{AppSessionId, SecureKey};
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
        app_session_id: AppSessionId,
        create:         CreateSession,
    },
    SetSpec {
        app_session_id: AppSessionId,
        spec:           SessionSpec,
    },
    SetSecurity {
        app_session_id: AppSessionId,
        security:       HashMap<SecureKey, SessionSecurity>,
    },
    Modify {
        app_session_id: AppSessionId,
        modifications:  Vec<ModifySessionSpec>,
    },
    SetDesiredPlayState {
        app_session_id:     AppSessionId,
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
