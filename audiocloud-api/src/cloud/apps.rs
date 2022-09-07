//! Cloud APIs for apps

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::AppId;

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct GetAppResponse {
    pub id:          AppId,
    pub enabled:     bool,
    pub admin_email: String,
    pub media_url:   String,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct UpdateApp {
    pub enabled:     Option<bool>,
    pub admin_email: Option<String>,
    pub media_url:   Option<String>,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AppUpdated {
    Updated(AppId),
}

#[utoipa::path(
  get,
  path = "/v1/apps/{app_id}", 
  responses(
    (status = 200, description = "Success", body = GetAppResponse),
    (status = 401, description = "Not authorized", body = CloudError),
    (status = 404, description = "Not found", body = CloudError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App to get")
  ))]
pub(crate) fn get() {}

#[utoipa::path(
  patch,
  path = "/v1/apps/{app_id}",
  request_body = UpdateApp,
  responses(
    (status = 200, description = "Success", body = AppUpdated),
    (status = 401, description = "Not authorized", body = CloudError),
    (status = 404, description = "Not found", body = CloudError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App to update")
  )
)]
pub(crate) fn update() {}
