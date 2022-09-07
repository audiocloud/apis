//! The API to the audio engine (from the domain side)

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

use crate::common::media::PlayId;

pub mod command;
pub mod event;

use crate::{AppId, AppMediaObjectId, AppTaskId, FixedInstanceId, MediaObject, ModifyTaskError, TaskId, TaskPlayState, TaskSpec};
pub use command::*;
pub use event::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CompressedAudio {
    pub play_id:      PlayId,
    pub timeline_pos: f64,
    pub stream_pos:   u64,
    #[serde(with = "serde_bytes")]
    pub buffer:       Vec<u8>,
    pub last:         bool,
}

#[derive(Debug, Error, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AudioEngineError {
    #[error("Track {0} not found")]
    TrackNotFound(usize),

    #[error("Item {0} on track {1} not found")]
    ItemNotFound(usize, usize),

    #[error("Task {0} failed to modify: {1}")]
    ModifyTask(AppTaskId, ModifyTaskError),

    #[error("Internal sound engine error: {0}")]
    InternalError(String),

    #[error("Remote call failed: {0}")]
    RPC(String),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskReplaced {
    Updated { app_id: AppId, task_id: TaskId },
    Created { app_id: AppId, task_id: TaskId },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TaskDeleted {
    Deleted { app_id: AppId, task_id: TaskId },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MediaUpdated {
    Updated { added: usize, replaced: usize, deleted: usize },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AudioEngineFixedInstance {
    pub input_start:  u32,
    pub output_start: u32,
    pub num_inputs:   u32,
    pub num_outputs:  u32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SetInstances {
    instances: HashMap<FixedInstanceId, AudioEngineFixedInstance>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SetMedia {
    media: HashMap<AppMediaObjectId, MediaObject>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InstancesUpdated {
    Updated { added: usize, replaced: usize, deleted: usize },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TaskWithStatus {
    pub id:         AppTaskId,
    pub spec:       TaskSpec,
    pub play_state: TaskPlayState,
    // TODO: add more fields here, when reviewing audio_engine REST implementation
}

pub type TaskWithStatusList = Vec<TaskWithStatus>;

pub mod tasks {
    /// Create or update task spec
    ///
    /// Create or update a task by providing its spec. Changing the spec even trivially could result
    /// in a rendering or playback interruption.
    #[utoipa::path(
     put,
     path = "/v1/tasks/{app_id}/{task_id}",
     request_body = TaskSpec,
     responses(
      (status = 200, description = "Success", body = TaskReplaced),
      (status = 401, description = "Not authorized", body = AudioEngineError),
     ),
     params(
      ("app_id" = AppId, Path, description = "App id"),
      ("task_id" = TaskId, Path, description = "Task id")
     ))]
    pub(crate) fn set_spec() {}

    /// Modify task spec
    ///
    /// Apply a modification to an existing spec. Changing the spec even trivially could result in a
    /// rendering or playback interruption. The task must exist in order to be modified.
    #[utoipa::path(
     patch,
     path = "/v1/tasks/{app_id}/{task_id}",
     request_body = ModifyTaskSpec,
     responses(
      (status = 200, description = "Success", body = TaskReplaced),
      (status = 400, description = "Modification failed", body = AudioEngineError),
      (status = 401, description = "Not authorized", body = AudioEngineError),
      (status = 404, description = "Not found", body = AudioEngineError),
     ),
     params(
      ("app_id" = AppId, Path, description = "App id"),
      ("task_id" = TaskId, Path, description = "Task id")
     ))]
    pub(crate) fn modify_spec() {}

    /// Delete a task
    ///
    /// Delete an existing task spec. This will interrupt any playback or rendering and will free
    /// resources associated wit hthe task (such as instances or locks on media files).
    #[utoipa::path(
     delete,
     path = "/v1/tasks/{app_id}/{task_id}",
     responses(
      (status = 200, description = "Success", body = TaskDeleted),
      (status = 401, description = "Not authorized", body = AudioEngineError),
      (status = 404, description = "Not found", body = AudioEngineError),
     ),
     params(
      ("app_id" = AppId, Path, description = "App id"),
      ("task_id" = TaskId, Path, description = "Task id")
     ))]
    pub(crate) fn delete() {}

    /// List tasks
    ///
    /// Return a list of all current tasks and their play status.
    #[utoipa::path(
     get,
     path = "/v1/tasks",
     responses(
      (status = 200, description = "Success", body = TaskWithStatusList),
      (status = 401, description = "Not authorized", body = AudioEngineError),
      (status = 404, description = "Not found", body = AudioEngineError),
     ))]
    pub(crate) fn list() {}

    /// Start playing a task
    ///
    /// Start playing a task that is stopped. The request will return when the task has started to play
    /// or with an error.
    #[utoipa::path(
      post,
      path = "/v1/tasks/{app_id}/{task_id}/transport/play",
      request_body = RequestPlay,
      responses(
        (status = 200, description = "Success", body = TaskWithStatusList),
        (status = 401, description = "Not authorized", body = AudioEngineError),
        (status = 404, description = "Not found", body = AudioEngineError),
      ),
      params(
        ("app_id" = AppId, Path, description = "App id"),
        ("task_id" = TaskId, Path, description = "Task id")
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
        (status = 200, description = "Success", body = TaskWithStatusList),
        (status = 401, description = "Not authorized", body = AudioEngineError),
        (status = 404, description = "Task Not found", body = AudioEngineError),
      ),
      params(
        ("app_id" = AppId, Path, description = "App id"),
        ("task_id" = TaskId, Path, description = "Task id")
      ))]
    pub(crate) fn seek() {}

    /// Change the selected mixer
    ///
    /// If the task is playing, change the mixer that is used to derive monitoring.
    #[utoipa::path(
      post,
      path = "/v1/tasks/{app_id}/{task_id}/transport/mixer",
      request_body = RequestChangeMixer,
      responses(
        (status = 200, description = "Success", body = TaskWithStatusList),
        (status = 401, description = "Not authorized", body = AudioEngineError),
        (status = 404, description = "Task or mixer Not found", body = AudioEngineError),
      ),
      params(
        ("app_id" = AppId, Path, description = "App id"),
        ("task_id" = TaskId, Path, description = "Task id")
      ))]
    pub(crate) fn change_mixer() {}

    /// Stop playing a task
    ///
    /// Request to stop a track if the task is playing.
    #[utoipa::path(
      post,
      path = "/v1/tasks/{app_id}/{task_id}/transport/stop",
      request_body = RequestStopPlay,
      responses(
        (status = 200, description = "Success", body = TaskWithStatusList),
        (status = 401, description = "Not authorized", body = AudioEngineError),
        (status = 404, description = "Task or mixer Not found", body = AudioEngineError),
      ),
      params(
        ("app_id" = AppId, Path, description = "App id"),
        ("task_id" = TaskId, Path, description = "Task id")
      ))]
    pub(crate) fn stop_playing() {}

    /// Cancel rendering a task
    ///
    /// Request to stop (cancel) rendering if the task is rendering.
    #[utoipa::path(
      post,
      path = "/v1/tasks/{app_id}/{task_id}/transport/cancel",
      request_body = RequestCancelRender,
      responses(
        (status = 200, description = "Success", body = TaskWithStatusList),
        (status = 401, description = "Not authorized", body = AudioEngineError),
        (status = 404, description = "Task or mixer Not found", body = AudioEngineError),
      ),
      params(
        ("app_id" = AppId, Path, description = "App id"),
        ("task_id" = TaskId, Path, description = "Task id")
      ))]
    pub(crate) fn cancel_render() {}

    /// Render a task to a new file
    ///
    /// Start rendering a task. Note that unlike the orchestration or domain API, the audio engine
    /// does not care if the media files are present and will happily execute a render even when no
    /// files (or instances) are ready. The caller to this API should make sure that any such
    /// preconditions are met.
    #[utoipa::path(
      post,
      path = "/v1/tasks/{app_id}/{task_id}/transport/render",
      request_body = RequestRender,
      responses(
        (status = 200, description = "Success", body = TaskWithStatusList),
        (status = 401, description = "Not authorized", body = AudioEngineError),
        (status = 404, description = "Task or mixer Not found", body = AudioEngineError),
      ),
      params(
        ("app_id" = AppId, Path, description = "App id"),
        ("task_id" = TaskId, Path, description = "Task id")
      ))]
    pub(crate) fn render() {}
}

pub mod environment {
    /// Set media presence
    ///
    /// The Audio Engine needs to map AppMediaObjectId on track items to
    #[utoipa::path(
     put,
     path = "/v1/media",
     request_body = SetMedia,
     responses(
      (status = 200, description = "Success", body = MediaUpdated),
      (status = 401, description = "Not authorized", body = AudioEngineError),
      (status = 404, description = "Not found", body = AudioEngineError),
     ))]
    pub(crate) fn set_media() {}

    /// Set instance I/O mapping
    ///
    /// The Audio Engine needs to map FixedInstanceNode to I/O on the audio interface it is bound
    /// to. For example, an instance may be bound to channels 0 and 1 or to channels 5 and 6 and
    /// the Audio Engine needs to know to route the audio correctly.
    #[utoipa::path(
     put,
     path = "/v1/instances",
     request_body = SetInstances,
     responses(
      (status = 200, description = "Success", body = InstancesUpdated),
      (status = 401, description = "Not authorized", body = AudioEngineError),
      (status = 404, description = "Not found", body = AudioEngineError),
     ))]
    pub(crate) fn set_instances() {}
}
