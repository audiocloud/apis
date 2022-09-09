use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::common::change::ModifyTask;
use crate::common::task::{DynamicInstanceNode, FixedInstanceNode, MixerNode, NodeConnection, TaskPermissions, TrackNode};
use crate::time::{TimeRange, Timestamp};
use crate::{
    AppId, AppTaskId, DomainId, DynamicInstanceNodeId, FixedInstanceNodeId, MixerNodeId, NodeConnectionId, SecureKey, TaskId, TrackNodeId,
};

/// Create a task
///
/// Tasks describe graphs of media operations that may execute in real time or unattended as a render.
/// They are allocated to a domain and an engine within that domain. Operations are executed with
/// the help of instances, which are fixed hardware blocks or dynamically instanced software
/// components.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct CreateTask {
    /// Domain that will be executing the task
    pub domain:      DomainId,
    /// When is the task reserving resources
    pub time:        TimeRange,
    /// Track nodes within the task
    #[serde(default)]
    pub tracks:      HashMap<TrackNodeId, TrackNode>,
    /// Mixer nodes within the task
    #[serde(default)]
    pub mixers:      HashMap<MixerNodeId, MixerNode>,
    /// Dynamic instance nodes within the task
    #[serde(default)]
    pub dynamic:     HashMap<DynamicInstanceNodeId, DynamicInstanceNode>,
    /// Fixed nodes within the task
    #[serde(default)]
    pub fixed:       HashMap<FixedInstanceNodeId, FixedInstanceNode>,
    /// Connections between nodes
    #[serde(default)]
    pub connections: HashMap<NodeConnectionId, NodeConnection>,
    /// Security keys associated with the task
    #[serde(default)]
    pub security:    HashMap<SecureKey, TaskPermissions>,
    /// If true, task creation will be evaluated for correctness, but no actual task will be created
    pub dry_run:     bool,
}

/// Task created successfully
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskCreated {
    /// Created normally
    Created {
        /// App creating the task
        app_id:  AppId,
        /// Task Id
        task_id: TaskId,
    },
    /// Validated successfully, but not created
    DryRun {
        /// App creating the task
        app_id:  AppId,
        /// Task Id
        task_id: TaskId,
    },
}

/// Task was updated successfully
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskUpdated {
    /// Updated normally
    Updated {
        /// App creating the task
        app_id:  AppId,
        /// Task Id
        task_id: TaskId,
        /// New version to be used with `If-Matches` when submitting further modifications
        version: u64,
    },
}

/// Task was deleted successfully
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskDeleted {
    /// Deleted normally
    Deleted {
        /// App creating the task
        app_id:  AppId,
        /// Task Id
        task_id: TaskId,
        /// Version when deleted
        version: u64,
    },
}

/// Adjust the task time
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct AdjustTaskTime {
    /// If not null, overwrite the starting time
    pub from: Option<Timestamp>,
    /// If not null, overwrite the ending time
    pub to:   Option<Timestamp>,
}

/// A list of tasks
pub type ModifyTaskList = Vec<ModifyTask>;

#[utoipa::path(
  post,
  path = "/v1/apps/{app_id}/tasks",
  request_body = CreateTask,
  responses(
    (status = 200, description = "Success", body = TaskCreated),
    (status = 401, description = "Not authorized", body = CloudError),
    (status = 404, description = "App not found", body = CloudError),
    (status = 409, description = "Overlapping task exists", body = CloudError),
  ),
  params(
    ("app_id" = AppId, Path, description = "The app for which we are creating a task")
  ))]
pub(crate) fn create_task() {}

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
pub(crate) fn modify_task_spec() {}

#[utoipa::path(
  put,
  path = "/v1/apps/{app_id}/tasks/{task_id}/time",
  request_body = AdjustTaskTime,
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
  ))]
pub(crate) fn adjust_task_time() {}

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
pub(crate) fn delete_task() {}
