//! Communication with the on-site media library

use crate::newtypes::{AppId, AppMediaObjectId, MediaObjectId};
use crate::session::{SessionTrackChannels, SessionTrackMediaFormat};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    pub notify_url:  String,
    // typescript: any
    pub context:     Value,
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
    pub notify_url: String,
    // typescript: any
    pub context:    Value,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ImportInDomain {
    pub path:        String,
    pub channels:    SessionTrackChannels,
    pub format:      SessionTrackMediaFormat,
    pub seconds:     f64,
    pub sample_rate: usize,
    pub bytes:       u64,
}

impl ImportInDomain {
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
    pub download: Option<MediaDownloadState>,
    pub upload:   Option<MediaUploadState>,
}
