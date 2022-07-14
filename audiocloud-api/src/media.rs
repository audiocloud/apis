//! Communication with the on-site media library

use crate::newtypes::{AppId, MediaObjectId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaDownloadState {
    Pending,
    Downloading {
        progress: f64,
        retry: usize,
    },
    Completed,
    Failed {
        error: String,
        count: usize,
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
        retry: usize,
    },
    Completed,
    Failed {
        error: String,
        count: usize,
        will_retry: bool,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DownloadMedia {
    pub get_url: String,
    pub notify_url: String,
    pub context: String,
}
