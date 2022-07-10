use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::media::{MediaDownloadState, MediaUploadState};
use crate::newtypes::DomainId;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppMedia {
  pub placements: HashMap<DomainId, MediaPlacement>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MediaPlacement {
  pub download: Option<MediaDownloadState>,
  pub uploaded: Option<MediaUploadState>,
}

// TODO: reports from media library that we have done something with the app file placements
