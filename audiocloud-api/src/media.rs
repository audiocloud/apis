//! Communication with the on-site media library

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::newtypes::{AppMediaObjectId, AppSessionId};
use crate::session::{SessionTrackChannels, SessionTrackMediaFormat};
use crate::time::{now, Timestamp, Timestamped};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MediaJobState {
    pub progress:    f64,
    pub retry:       usize,
    pub error:       Option<String>,
    pub in_progress: bool,
    pub updated_at:  Timestamp,
}

impl Default for MediaJobState {
    fn default() -> Self {
        Self { progress:    0.0,
               retry:       0,
               error:       None,
               in_progress: false,
               updated_at:  now(), }
    }
}

impl MediaJobState {
    pub fn is_finished_ok(&self) -> bool {
        !self.in_progress && self.error.is_none()
    }
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MediaDownload {
    pub download: DownloadFromDomain,
    pub state:    MediaJobState,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MediaUpload {
    pub upload: UploadToDomain,
    pub state:  MediaJobState,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaObject {
    pub id:       AppMediaObjectId,
    pub metadata: Option<MediaMetadata>,
    pub path:     Option<String>,
    pub download: Option<MediaDownload>,
    pub upload:   Option<MediaUpload>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateMediaSession {
    pub media_objects: HashSet<AppMediaObjectId>,
    pub ends_at:       Timestamp,
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
