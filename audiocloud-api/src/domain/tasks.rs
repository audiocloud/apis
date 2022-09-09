use crate::{
  AppMediaObjectId, AppTaskId, FixedInstanceId, InstancePlayState, MediaObject, ModifyTaskSpec, PlayId, RenderId, SecureKey,
  TaskPlayState, TaskPermissions, TaskSpec, TimeRange,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TaskSummary {
    pub id:                    AppTaskId,
    pub play_state:            TaskPlayState,
    pub waiting_for_instances: HashSet<FixedInstanceId>,
    pub waiting_for_media:     HashSet<AppMediaObjectId>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TaskWithStatusAndSpec {
    pub id:         AppTaskId,
    pub play_state: TaskPlayState,
    pub instances:  HashMap<FixedInstanceId, InstancePlayState>,
    pub media:      HashMap<AppMediaObjectId, MediaObject>,
    pub spec:       TaskSpec,
}

pub type TaskSummaryList = Vec<TaskSummary>;

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct CreateTask {
    pub id:       AppTaskId,
    pub spec:     TaskSpec,
    pub time:     TimeRange,
    pub security: HashMap<SecureKey, TaskPermissions>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskCreated {
    Created { id: AppTaskId },
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct ModifyTask {
    pub modify_spec: Vec<ModifyTaskSpec>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskModified {
    Modified { id: AppTaskId },
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskDeleted {
    Deleted { id: AppTaskId },
}

pub use crate::audio_engine::{TaskPlayStopped, TaskPlaying, TaskRenderCancelled, TaskRendering, TaskSought};

/// List tasks
///
/// Return a list of all current tasks and their status.
#[utoipa::path(
  get,
  path = "/v1/tasks",
  responses(
    (status = 200, description = "Success", body = TaskSummaryList),
    (status = 401, description = "Not authorized", body = DomainError),
  ))]
pub(crate) fn list() {}

/// Get task details
///
/// Get details of a task, including dependent media and instance statuses
#[utoipa::path(
  get,
  path = "/v1/tasks/{app_id}/{task_id}",
  responses(
    (status = 200, description = "Success", body = TaskWithStatusAndSpec),
    (status = 401, description = "Not authorized", body = DomainError),
    (status = 404, description = "Not found", body = DomainError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id")
  ))]
pub(crate) fn get() {}

/// Create a task
///
/// In standalone mode, the task will be checked for mutual exclusivity with other tasks, otherwise
/// it will be created. This call could also fail if the referenced resources (such as fixed
/// instances) do not exist.
#[utoipa::path(
  post,
  path = "/v1/tasks",
  request_body = CreateTask,
  responses(
    (status = 200, description = "Success", body = TaskCreated),
    (status = 401, description = "Not authorized", body = DomainError),
    (status = 404, description = "Not found", body = DomainError),
    (status = 409, description = "Overlapping task exists", body = DomainError),
  ))]
pub(crate) fn create() {}

/// Modify existing task
///
/// Submit modifications to the task. This generic request can be used to update most aspects of the
/// session: adjusting parameters, creating, deleting, reconnecting nodes, changing media, etc.
#[utoipa::path(
  post,
  path = "/v1/tasks/{app_id}/{task_id}/modify",
  request_body = ModifyTask,
  responses(
    (status = 200, description = "Success", body = TaskModified),
    (status = 401, description = "Not authorized", body = DomainError),
    (status = 404, description = "Not found", body = DomainError),
    (status = 409, description = "Not allowed to change instances", body = DomainError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id"),
    ("If-Match" = u64, Header, description = "The task version to be changed"),
  ))]
pub(crate) fn modify() {}

/// Delete a task
///
/// Delete a task and release all connected resources.
#[utoipa::path(
  delete,
  path = "/v1/tasks/{app_id}/{task_id}",
  responses(
    (status = 200, description = "Success", body = TaskDeleted),
    (status = 401, description = "Not authorized", body = DomainError),
    (status = 404, description = "Not found", body = DomainError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id")
  ))]
pub(crate) fn delete() {}

/// Render a task to a new file
///
/// The domain will check that
#[utoipa::path(
  post,
  path = "/v1/tasks/{app_id}/{task_id}/transport/render",
  request_body = RequestRender,
  responses(
    (status = 200, description = "Success", body = TaskRendering),
    (status = 401, description = "Not authorized", body = DomainError),
    (status = 404, description = "Task or mixer Not found", body = DomainError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id")
  ))]
pub(crate) fn render() {}

/// Start playing a task
///
/// Start playing a task that is stopped. The request will return when the task has started to play
/// or with an error.
#[utoipa::path(
  post,
  path = "/v1/tasks/{app_id}/{task_id}/transport/play",
  request_body = RequestPlay,
  responses(
    (status = 200, description = "Success", body = TaskPlaying),
    (status = 401, description = "Not authorized", body = DomainError),
    (status = 404, description = "Task or mixer Not found", body = DomainError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id"),
    ("If-Match" = u64, Header, description = "The task version"),
  ))]
pub(crate) fn play() {}

/// Seek while task is playing
///
/// If the task is playing, change the playing position.
#[utoipa::path(
  post,
  path = "/v1/tasks/{app_id}/{task_id}/transport/seek",
  request_body = RequestSeek,
  responses(
    (status = 200, description = "Success", body = TaskSought),
    (status = 401, description = "Not authorized", body = DomainError),
    (status = 404, description = "Task Not found", body = DomainError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id")
  ))]
pub(crate) fn seek() {}

/// Cancel rendering a task
///
/// Request to stop (cancel) rendering if the task is rendering.
#[utoipa::path(
  post,
  path = "/v1/tasks/{app_id}/{task_id}/transport/cancel",
  request_body = RequestCancelRender,
  responses(
    (status = 200, description = "Success", body = TaskRenderCancelled),
    (status = 401, description = "Not authorized", body = DomainError),
    (status = 404, description = "Task or mixer Not found", body = DomainError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id"),
    ("If-Match" = u64, Header, description = "The task version"),
  ))]
pub(crate) fn cancel_render() {}

/// Stop playing a task
///
/// Request to stop a track if the task is playing.
#[utoipa::path(
  post,
  path = "/v1/tasks/{app_id}/{task_id}/transport/stop",
  request_body = RequestStopPlay,
  responses(
    (status = 200, description = "Success", body = TaskPlayStopped),
    (status = 401, description = "Not authorized", body = DomainError),
    (status = 404, description = "Task or mixer Not found", body = DomainError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id"),
    ("If-Match" = u64, Header, description = "The task version"),
  ))]
pub(crate) fn stop_playing() {}
