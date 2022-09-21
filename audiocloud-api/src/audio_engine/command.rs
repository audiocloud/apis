use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{AppMediaObjectId, AppTaskId, DynamicInstanceNodeId, FixedInstanceId};
use crate::cloud::domains::InstanceRouting;
use crate::common::change::{ModifyTaskSpec, UpdateTaskPlay};
use crate::common::media::{PlayId, RenderId, RequestPlay, RequestRender};
use crate::common::task::TaskSpec;

/// Command sent to the Audio Engine
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AudioEngineCommand {
    /// Set task specification.
    ///
    /// Create task if not already created.
    SetSpec {
        /// Task id
        task_id:     AppTaskId,
        /// Task specification
        spec:        TaskSpec,
        /// Current routing state for fixed instances
        instances:   HashMap<FixedInstanceId, InstanceRouting>,
        /// Current media state
        media_ready: HashMap<AppMediaObjectId, String>,
    },
    /// Update media state of a task
    Media {
        /// Task id
        task_id:     AppTaskId,
        /// Media state
        media_ready: HashMap<AppMediaObjectId, String>,
    },
    /// Update instance routing information for a task
    Instances {
        /// Task id
        task_id:   AppTaskId,
        /// Instance state
        instances: HashMap<FixedInstanceId, InstanceRouting>,
    },
    /// Modify a task specification
    ModifySpec {
        /// Task id
        task_id:     AppTaskId,
        /// List of changes
        transaction: Vec<ModifyTaskSpec>,
        /// Current routing state for fixed instances
        instances:   HashMap<FixedInstanceId, InstanceRouting>,
        /// Current media state
        media_ready: HashMap<AppMediaObjectId, String>,
    },
    /// Set parameters of a dynamic instance node
    SetDynamicParameterValues {
        /// Task id
        task_id:    AppTaskId,
        /// Dynamic instance node id
        dynamic_id: DynamicInstanceNodeId,
        /// Parameters to be set
        values:     serde_json::Value,
    },
    /// Render the task
    Render {
        /// Task id
        task_id: AppTaskId,
        /// Render request
        render:  RequestRender,
    },
    /// Play the task
    Play {
        /// Task id
        task_id: AppTaskId,
        /// Play request
        play:    RequestPlay,
    },
    /// Update play parameters while the task is playing
    UpdatePlay {
        /// Task id
        task_id: AppTaskId,
        /// Update of the play parameters
        update:  UpdateTaskPlay,
    },
    /// Stop rendering the task
    StopRender {
        /// Task id
        task_id:   AppTaskId,
        /// Render id
        render_id: RenderId,
    },
    /// Stop playing the task
    StopPlay {
        /// Task id
        task_id: AppTaskId,
        /// Play id
        play_id: PlayId,
    },
    /// Close (remove) the session
    Close {
        /// Task id
        task_id: AppTaskId,
    },
}
