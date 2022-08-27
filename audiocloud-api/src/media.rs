//! Communication with the on-site media library

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::newtypes::{AppMediaObjectId, AppSessionId, MediaObjectId};
use crate::session::{SessionTrackChannels, SessionTrackMediaFormat};
use crate::time::{Timestamp, Timestamped};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaDownloadState {
    Pending,
    Downloading {
        progress: f64,
        retry:    usize,
    },
    Completed,
    Failed {
        error:      String,
        count:      usize,
        will_retry: bool,
    },
    Evicted,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaUploadState {
    Pending,
    Uploading {
        progress: f64,
        retry:    usize,
    },
    Completed,
    Failed {
        error:      String,
        count:      usize,
        will_retry: bool,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DownloadMedia {
    pub get_url:    String,
    pub notify_url: String,
    pub context:    String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MediaMetadata {
    pub channels:    SessionTrackChannels,
    pub format:      SessionTrackMediaFormat,
    pub seconds:     f64,
    pub sample_rate: usize,
    pub bytes:       u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UploadToDomain {
    pub channels:    SessionTrackChannels,
    pub format:      SessionTrackMediaFormat,
    pub seconds:     f64,
    pub sample_rate: usize,
    pub bytes:       u64,
    pub url:         String,
    pub notify_url:  Option<String>,
    // typescript: any
    pub context:     Option<Value>,
}

impl UploadToDomain {
    pub fn metadata(&self) -> MediaMetadata {
        MediaMetadata { channels:    self.channels,
                        format:      self.format,
                        seconds:     self.seconds,
                        sample_rate: self.sample_rate,
                        bytes:       self.bytes, }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DownloadFromDomain {
    pub url:        String,
    pub notify_url: Option<String>,
    // typescript: any
    pub context:    Option<Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImportToDomain {
    pub path:        String,
    pub channels:    SessionTrackChannels,
    pub format:      SessionTrackMediaFormat,
    pub seconds:     f64,
    pub sample_rate: usize,
    pub bytes:       u64,
}

impl ImportToDomain {
    pub fn metadata(&self) -> MediaMetadata {
        MediaMetadata { channels:    self.channels,
                        format:      self.format,
                        seconds:     self.seconds,
                        sample_rate: self.sample_rate,
                        bytes:       self.bytes, }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaObject {
    pub id:       AppMediaObjectId,
    pub metadata: Option<MediaMetadata>,
    pub path:     Option<String>,
    pub download: Timestamped<MediaDownloadState>,
    pub upload:   Timestamped<MediaUploadState>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateMediaSession {
    pub media_objects: HashSet<AppMediaObjectId>,
    pub ends_at:       Timestamp,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MediaServiceEvent {
    SessionMediaState {
        session_id: AppSessionId,
        media:      HashMap<AppMediaObjectId, MediaObject>,
    },
}

impl MediaServiceEvent {
    pub fn session_id(&self) -> &AppSessionId {
        match self {
            MediaServiceEvent::SessionMediaState { session_id, .. } => session_id,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MediaServiceCommand {
    SetSessionMedia {
        session_id: AppSessionId,
        media:      HashSet<AppMediaObjectId>,
    },
    DeleteSession {
        session_id: AppSessionId,
    },
}
