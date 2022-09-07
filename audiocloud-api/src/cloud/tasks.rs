use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

use crate::common::change::ModifyTask;
use crate::common::task::{DynamicInstanceNode, FixedInstanceNode, MixerNode, NodeConnection, TaskSecurity, TrackNode};
use crate::time::{TimeRange, Timestamp};
use crate::{AppTaskId, DomainId, DynamicInstanceNodeId, FixedInstanceNodeId, MixerNodeId, NodeConnectionId, SecureKey, TrackNodeId};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct CreateTask {
    pub domain:      DomainId,
    pub time:        TimeRange,
    #[serde(default)]
    pub tracks:      HashMap<TrackNodeId, TrackNode>,
    #[serde(default)]
    pub mixers:      HashMap<MixerNodeId, MixerNode>,
    #[serde(default)]
    pub dynamic:     HashMap<DynamicInstanceNodeId, DynamicInstanceNode>,
    #[serde(default)]
    pub fixed:       HashMap<FixedInstanceNodeId, FixedInstanceNode>,
    #[serde(default)]
    pub connections: HashMap<NodeConnectionId, NodeConnection>,
    #[serde(default)]
    pub security:    HashMap<SecureKey, TaskSecurity>,
    pub dry_run:     bool,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskCreated {
    Created(AppTaskId),
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskUpdated {
    Updated(AppTaskId),
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskDeleted {
    Deleted(AppTaskId),
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct AdjustTaskTime {
    pub from: Option<Timestamp>,
    pub to:   Option<Timestamp>,
}

pub type ModifyTaskList = Vec<ModifyTask>;

#[utoipa::path(
  post,
  path = "/v1/apps/{app_id}/tasks",
  responses(
    (status = 200, description = "Success", body = TaskCreated),
    (status = 401, description = "Not authorized", body = CloudError),
    (status = 404, description = "App not found", body = CloudError),
    (status = 409, description = "Overlapping task exists", body = CloudError),
  ),
  params(
    ("app_id" = AppId, Path, description = "The app for which we are creating a task"),
    ("spec" = CreateTask, description = "The task spec"),
))]
pub(crate) fn create() {}

#[utoipa::path(
  put,
  path = "/v1/apps/{app_id}/tasks/{task_id}/spec",
  request_body = ModifyTaskList,
  responses(
    (status = 200, description = "Success", body = TaskUpdated),
    (status = 401, description = "Not authorized", body = CloudError),
    (status = 404, description = "App or task not found", body = CloudError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App owning the task"),
    ("task_id" = TaskId, Path, description = "Task to be updated"),
    ("If-Match" = u64, Header, description = "The task version for"),
  ))]
pub(crate) fn modify_spec() {}

#[utoipa::path(
  put,
  path = "/v1/apps/{app_id}/tasks/{task_id}/time",
  responses(
    (status = 200, description = "Success", body = TaskUpdated),
    (status = 401, description = "Not authorized", body = CloudError),
    (status = 404, description = "App or task not found", body = CloudError),
    (status = 409, description = "Overlapping task exists", body = CloudError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App owning the task"),
    ("task_id" = TaskId, Path, description = "Task to be updated"),
    ("If-Match" = u64, Header, description = "The task version for"),
    ("spec" = AdjustTaskTime, description = "The task spec"),
  ))]
pub(crate) fn adjust_time() {}

#[utoipa::path(
  delete,
  path = "/v1/apps/{app_id}/tasks/{task_id}",
  responses(
    (status = 200, description = "Success", body = TaskDeleted),
    (status = 401, description = "Not authorized", body = CloudError),
    (status = 404, description = "App not found", body = CloudError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App owning the task"),
    ("task_id" = TaskId, Path, description = "Task to be deleted"),
  ))]
pub(crate) fn delete() {}
