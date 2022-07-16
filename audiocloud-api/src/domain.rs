//! API definitions for communicating with the domain
//!
//! The domain will communicate with either apps that connect directly to them
//! or with the cloud.

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::newtypes::{AppMediaObjectId, AppSessionId, SecureKey};
use crate::session::SessionSecurity;
use crate::{
    app::SessionPacket,
    change::{DesiredSessionPlayState, ModifySessionSpec},
    cloud::apps::{CreateSession, SessionSpec},
    media::{DownloadMedia, MediaDownloadState},
    newtypes::{MediaObjectId, SessionId},
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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DomainMediaCommand {
    Download {
        app_media_id: AppMediaObjectId,
        download:     DownloadMedia,
    },
    Delete {
        app_media_id: AppMediaObjectId,
    },
}

impl DomainMediaCommand {
    pub fn get_app_media_object_id(&self) -> &AppMediaObjectId {
        match self {
            Self::Download { app_media_id, .. } => app_media_id,
            Self::Delete { app_media_id } => app_media_id,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum WebSocketEvent {
    Packet(SessionId, Arc<SessionPacket>),
    Download(MediaObjectId, MediaDownloadState),
}
