use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{AppMediaObjectId, AppTaskId, DynamicInstanceNodeId, FixedInstanceId, MultiChannelValue, ParameterId};
use crate::common::change::{ModifyTaskSpec, UpdateTaskPlay};
use crate::common::task::TaskSpec;
use crate::cloud::domains::InstanceRouting;
use crate::common::media::{PlayId, RenderId, RequestPlay, RequestRender};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AudioEngineCommand {
    SetSpec {
        session_id: AppTaskId,
        spec: TaskSpec,
        instances:   HashMap<FixedInstanceId, InstanceRouting>,
        media_ready: HashMap<AppMediaObjectId, String>,
    },
    Media {
        session_id: AppTaskId,
        media_ready: HashMap<AppMediaObjectId, String>,
    },
    Instances {
        session_id: AppTaskId,
        instances:  HashMap<FixedInstanceId, InstanceRouting>,
    },
    ModifySpec {
        session_id: AppTaskId,
        transaction: Vec<ModifyTaskSpec>,
        instances:   HashMap<FixedInstanceId, InstanceRouting>,
        media_ready: HashMap<AppMediaObjectId, String>,
    },
    SetDynamicParameters {
        session_id: AppTaskId,
        dynamic_id: DynamicInstanceNodeId,
        parameters: HashMap<ParameterId, MultiChannelValue>,
    },
    Render {
        session_id: AppTaskId,
        render: RequestRender,
    },
    Play {
        session_id: AppTaskId,
        play: RequestPlay,
    },
    UpdatePlay {
        session_id: AppTaskId,
        update: UpdateTaskPlay,
    },
    StopRender {
        session_id: AppTaskId,
        render_id:  RenderId,
    },
    StopPlay {
        session_id: AppTaskId,
        play_id:    PlayId,
    },
    Close {
        session_id: AppTaskId,
    },
}
