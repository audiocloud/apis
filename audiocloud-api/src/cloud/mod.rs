//! API definitions for the Cloud

use std::collections::HashSet;

use schemars::schema::RootSchema;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::OpenApi;

use crate::common::change::ModifyTaskError;
use crate::common::model::ResourceId;
use crate::{
    merge_schemas, AppId, AppMediaObjectId, AppTaskId, DomainId, DynamicInstanceNodeId, FixedInstanceId, FixedInstanceNodeId, ModelId,
};

pub mod apps;
pub mod domains;
pub mod media;
pub mod models;
pub mod tasks;

#[derive(Serialize, Deserialize, Debug, Clone, Error, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CloudError {
    #[error("API Key not found")]
    ApiKeyNotFound,

    #[error("App file {0} not found")]
    AppFileNotFound(AppMediaObjectId),

    #[error("App not found")]
    AppNotFound(AppId),

    #[error("{0} is an invalid app task ID")]
    InvalidAppTaskId(String),

    #[error("{0} is an invalid app media object ID")]
    InvalidAppMediaObjectId(String),

    #[error("At least a segment of a reservation needs to be in the future")]
    OnlyFutureReservations,

    #[error("Task time must be well-formed")]
    TimeMalformed,

    #[error("Task duration is smaller than domain minimum task duration time {0} ms")]
    DurationTooShort(f64),

    #[error("Too many tasks reserved on domain")]
    TooManyTasks,

    #[error("Detected internal inconsistency: {0}")]
    InternalInconsistency(String),

    #[error("Instances overlapping: {0:?}")]
    OverlappingFixedInstances(HashSet<FixedInstanceId>),

    #[error("Domain {0} unknown")]
    DomainNotFound(DomainId),

    #[error("Instance {0} unknown")]
    InstanceNotFound(FixedInstanceId),

    #[error("Instance {1} required by bus {0} but not reserved")]
    InstanceNotReferenced(u64, FixedInstanceId),

    #[error("Model {2} of a dynamic instance required by bus {0} is not supported on domain {1}")]
    DynamicInstanceNotSupported(DynamicInstanceNodeId, DomainId, ModelId),

    #[error("Fixed instance {2} required by bus {0} is not supported on domain {1}")]
    FixedInstanceNotSupported(FixedInstanceNodeId, DomainId, FixedInstanceId),

    #[error("Fixed instance {2} required by bus {0} is not assignable to app {3} on domain {1}")]
    FixedInstanceAccessDenied(FixedInstanceNodeId, DomainId, FixedInstanceId, AppId),

    #[error("Out of {0} resource by {1}")]
    OutOfResource(ResourceId, f64),

    #[error("Object source {0} on bus {1} is too short-lived")]
    ObjectTooShortLived(DomainId, u64, u64),

    #[error("Task {0} was not found")]
    TaskNotFound(AppTaskId),

    #[error("Task could not be modified: {0}")]
    TaskModification(#[from] ModifyTaskError),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Authorization failed: {0}")]
    Authorization(String),

    #[error("All retries exhausted while trying to obtain a lock")]
    BlockingLock,
}

#[derive(OpenApi)]
#[openapi(paths(apps::get,
                apps::update,
                tasks::create,
                tasks::adjust_time,
                tasks::delete,
                tasks::modify_spec,
                domains::get,
                domains::boot,
                domains::add_maintenance,
                domains::delete_maintenance,
                domains::add_fixed_instance_maintenance,
                domains::delete_fixed_instance_maintenance,
                media::upload_media_object,
                media::download_media_object,
                media::delete_media_object,
                media::report_media_job_progress))]
pub struct CloudApi;

pub fn schemas() -> RootSchema {
    merge_schemas([schema_for!(CloudError),
                   schema_for!(crate::ModifyTaskError),
                   schema_for!(crate::AppId),
                   schema_for!(crate::DomainId),
                   schema_for!(crate::TaskId),
                   schema_for!(crate::TimeRange),
                   schema_for!(crate::TrackNode),
                   schema_for!(crate::MixerNode),
                   schema_for!(crate::DynamicInstanceNode),
                   schema_for!(crate::FixedInstanceNode),
                   schema_for!(crate::NodeConnection),
                   schema_for!(crate::TaskSecurity),
                   schema_for!(crate::TrackMedia),
                   schema_for!(crate::TaskSpec),
                   schema_for!(crate::ModifyTaskSpec),
                   schema_for!(crate::ModifyTask),
                   schema_for!(crate::Model),
                   schema_for!(crate::MediaJobState),
                   schema_for!(crate::UploadToDomain),
                   schema_for!(crate::DownloadFromDomain),
                   schema_for!(apps::GetAppResponse),
                   schema_for!(apps::UpdateApp),
                   schema_for!(apps::AppUpdated),
                   schema_for!(tasks::CreateTask),
                   schema_for!(tasks::TaskCreated),
                   schema_for!(tasks::TaskUpdated),
                   schema_for!(tasks::TaskDeleted),
                   schema_for!(tasks::AdjustTaskTime),
                   schema_for!(tasks::ModifyTaskList),
                   schema_for!(domains::DomainMediaInstanceSettings),
                   schema_for!(domains::DomainPowerInstanceSettings),
                   schema_for!(domains::GetDomainResponse),
                   schema_for!(domains::BootDomainResponse),
                   schema_for!(domains::DomainUpdated),
                   schema_for!(domains::AddMaintenance),
                   schema_for!(domains::DeleteMaintenance),
                   schema_for!(domains::Maintenance),
                   schema_for!(domains::AppFixedInstance),
                   schema_for!(domains::DomainFixedInstance),
                   schema_for!(domains::DynamicInstanceLimits),
                   schema_for!(domains::DomainLimits),
                   schema_for!(media::DownloadCreated),
                   schema_for!(media::UploadCreated),
                   schema_for!(media::MediaObjectDeleted),
                   schema_for!(media::ReportMediaJobProgress)].into_iter())
}
