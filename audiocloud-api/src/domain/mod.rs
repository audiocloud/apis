//! API definitions for communicating with the domain
//!
//! The domain will communicate with either apps that connect directly to them
//! or with the cloud.

use std::collections::HashMap;

use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::OpenApi;

use streaming::TaskStreamingPacket;

use crate::audio_engine::AudioEngineError;
use crate::cloud::tasks::CreateTask;
use crate::common::change::SessionState;
use crate::common::change::{DesiredTaskPlayState, ModifyTaskSpec};
use crate::common::error::SerializableResult;
use crate::common::task::TaskSecurity;
use crate::common::task::TaskSpec;
use crate::instance_driver::InstanceDriverError;
use crate::newtypes::{AppTaskId, SecureKey};
use crate::{merge_schemas, AppId, AudioEngineId, FixedInstanceId, TaskId};

pub mod streaming;
pub mod tasks;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DomainSessionCommand {
    Create {
        app_session_id: AppTaskId,
        create:         CreateTask,
    },
    SetSpec {
        app_session_id: AppTaskId,
        version:        u64,
        spec:           TaskSpec,
    },
    SetSecurity {
        app_session_id: AppTaskId,
        version:        u64,
        security:       HashMap<SecureKey, TaskSecurity>,
    },
    Modify {
        app_session_id: AppTaskId,
        version:        u64,
        modifications:  Vec<ModifyTaskSpec>,
    },
    SetDesiredPlayState {
        app_session_id:     AppTaskId,
        version:            u64,
        desired_play_state: DesiredTaskPlayState,
    },
    Delete {
        app_session_id: AppTaskId,
    },
}

impl DomainSessionCommand {
    pub fn get_session_id(&self) -> &AppTaskId {
        match self {
            DomainSessionCommand::Create { app_session_id, .. } => app_session_id,
            DomainSessionCommand::SetSpec { app_session_id, .. } => app_session_id,
            DomainSessionCommand::SetSecurity { app_session_id, .. } => app_session_id,
            DomainSessionCommand::Modify { app_session_id, .. } => app_session_id,
            DomainSessionCommand::SetDesiredPlayState { app_session_id, .. } => app_session_id,
            DomainSessionCommand::Delete { app_session_id, .. } => app_session_id,
        }
    }

    pub fn get_kind(&self) -> &'static str {
        match self {
            DomainSessionCommand::Create { .. } => "create",
            DomainSessionCommand::SetSpec { .. } => "set_spec",
            DomainSessionCommand::SetSecurity { .. } => "set_security",
            DomainSessionCommand::Modify { .. } => "modify",
            DomainSessionCommand::SetDesiredPlayState { .. } => "set_desired_play_state",
            DomainSessionCommand::Delete { .. } => "delete",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum WebSocketEvent {
    Packet(AppTaskId, TaskStreamingPacket),
    Spec(AppTaskId, TaskSpec),
    State(AppTaskId, SessionState),
    SessionError(AppTaskId, String),
    Response(String, SerializableResult<()>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum WebSocketCommand {
    Login(AppTaskId, SecureKey),
    Logout(AppTaskId),
    Session(DomainSessionCommand),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebSocketCommandEnvelope {
    pub request_id: String,
    pub command:    WebSocketCommand,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Error)]
pub enum DomainError {
    #[error("Instance driver for instance {0}: {1}")]
    InstanceDriver(FixedInstanceId, InstanceDriverError),

    #[error("Audio engine {0}: {1}")]
    AudioEngine(AudioEngineId, AudioEngineError),
}

#[derive(OpenApi)]
#[openapi(paths(tasks::list,
                tasks::get,
                tasks::create,
                tasks::modify,
                tasks::delete,
                tasks::render,
                tasks::play,
                tasks::seek,
                tasks::cancel_render,
                tasks::stop_playing,
                streaming::stream_packets))]
pub struct DomainApi;

pub fn schemas() -> RootSchema {
    merge_schemas([schema_for!(DomainError),
                   schema_for!(AppId),
                   schema_for!(TaskId),
                   schema_for!(tasks::TaskSummaryList),
                   schema_for!(tasks::TaskWithStatusAndSpec),
                   schema_for!(tasks::CreateTask),
                   schema_for!(tasks::ModifyTask),
                   schema_for!(tasks::TaskCreated),
                   schema_for!(tasks::TaskDeleted),
                   schema_for!(tasks::TaskModified),
                   schema_for!(tasks::TaskPlayStopped),
                   schema_for!(tasks::TaskPlaying),
                   schema_for!(tasks::TaskRenderCancelled),
                   schema_for!(tasks::TaskRendering),
                   schema_for!(tasks::TaskSought),
                   schema_for!(streaming::TaskStreamingPacket),
                   schema_for!(crate::RequestPlay),
                   schema_for!(crate::RequestSeek),
                   schema_for!(crate::RequestChangeMixer),
                   schema_for!(crate::RequestStopPlay),
                   schema_for!(crate::RequestCancelRender)].into_iter())
}
