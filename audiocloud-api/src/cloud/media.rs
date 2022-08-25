use std::collections::{HashMap, HashSet};

use crate::change::RenderId;
use serde::{Deserialize, Serialize};

use crate::media::{MediaDownloadState, MediaUploadState};
use crate::newtypes::{AppId, DomainId, MediaObjectId, SessionId};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppMedia {
    pub placements: HashMap<DomainId, MediaPlacement>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaPlacement {
    pub download: Option<MediaDownloadState>,
    pub uploaded: Option<MediaUploadState>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateAppMedia {
    pub get_url:    String,
    #[serde(default)]
    pub grouping:   Option<String>,
    #[serde(default)]
    pub grouping2:  Option<String>,
    pub sync_to:    HashSet<DomainId>,
    pub context:    String,
    pub notify_url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateAppMedia {
    #[serde(default)]
    pub grouping:  Option<String>,
    #[serde(default)]
    pub grouping2: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QueryAppMedia {
    #[serde(default)]
    pub grouping_is:        Option<String>,
    #[serde(default)]
    pub grouping_contains:  Option<String>,
    #[serde(default)]
    pub grouping2_is:       Option<String>,
    #[serde(default)]
    pub grouping2_contains: Option<String>,
    #[serde(default)]
    pub domain_id:          Option<DomainId>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReportUploadState {
    pub app_id:     AppId,
    pub session_id: SessionId,
    pub render_id:  RenderId,
    pub media_id:   MediaObjectId,
    pub state:      MediaUploadState,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReportDownloadState {
    pub app_id:   AppId,
    pub media_id: MediaObjectId,
    pub state:    MediaDownloadState,
}
