use crate::AppMediaObjectId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::common::media::MediaJobState;
use crate::common::{AppId, DomainId, MediaObjectId, TaskId};

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReportMediaJobProgress {
    UploadFromDomain {
        app_id:   AppId,
        media_id: MediaObjectId,
        state:    MediaJobState,
    },
    DownloadToDomain {
        app_id:   AppId,
        task_id:  Option<TaskId>,
        media_id: MediaObjectId,
        state:    MediaJobState,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UploadCreated {
    Created { media_id: AppMediaObjectId, domain_id: DomainId },
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DownloadCreated {
    Created { media_id: AppMediaObjectId, domain_id: DomainId },
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MediaObjectDeleted {
    Deleted { media_id: AppMediaObjectId },
}

#[utoipa::path(
  put,
  path = "/v1/domains/{domain_id}/media/{app_id}/{object_id}/upload",
  request_body = UploadToDomain,
  responses(
    (status = 200, description = "Success", body = UploadCreated),
    (status = 401, description = "Not authorized", body = CloudError),
    (status = 404, description = "Domain, app or task not found", body = CloudError),
  ),
  params(
    ("domain_id" = DomainId, Path, description = "Domain to upload the file to"),
    ("app_id" = AppId, Path, description = "Owner of the file"),
    ("object_id" = MediaObjectId, Path, description = "File object ID"),
  ))]
pub(crate) fn upload_media_object() {}

#[utoipa::path(
  put,
  path = "/v1/domains/{domain_id}/media/{app_id}/{object_id}/download",
  request_body = DownloadFromDomain,
  responses(
    (status = 200, description = "Success", body = DownloadCreated),
    (status = 401, description = "Not authorized", body = CloudError),
    (status = 404, description = "Domain, app or task not found", body = CloudError),
  ),
  params(
    ("domain_id" = DomainId, Path, description = "Domain to download the file from"),
    ("app_id" = AppId, Path, description = "Owner of the file"),
    ("object_id" = MediaObjectId, Path, description = "File object ID"),
  ))]
pub(crate) fn download_media_object() {}

#[utoipa::path(
  delete,
  path = "/v1/apps/{app_id}/media/{object_id}",
  request_body = UploadToDomain,
  responses(
    (status = 200, description = "Success", body = MediaObjectDeleted),
    (status = 401, description = "Not authorized", body = CloudError),
    (status = 404, description = "Domain, app or task not found", body = CloudError),
  ),
  params(
    ("app_id" = AppId, Path, description = "Owner of the file"),
    ("object_id" = MediaObjectId, Path, description = "File object ID"),
  ))]
pub(crate) fn delete_media_object() {}

#[utoipa::path(
  put,
  path = "/v1/domains/{domain_id}/media/{app_id}/{object_id}/report",
  request_body = ReportMediaJobProgress,
  responses(
    (status = 200, description = "Success", body = DownloadCreated),
    (status = 401, description = "Not authorized", body = CloudError),
    (status = 404, description = "Domain, app or object not found", body = CloudError),
  ),
  params(
    ("domain_id" = DomainId, Path, description = "Domain to download the file from"),
    ("app_id" = AppId, Path, description = "Owner of the file"),
    ("object_id" = MediaObjectId, Path, description = "File object ID"),
  ))]
pub(crate) fn report_media_job_progress() {}
