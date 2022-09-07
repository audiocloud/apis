//! API definitions for the Cloud

use std::collections::HashSet;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common::change::ModifyTaskError;
use crate::common::model::ResourceId;
use crate::{AppId, AppMediaObjectId, AppTaskId, DomainId, DynamicInstanceNodeId, FixedInstanceId, FixedInstanceNodeId, ModelId};

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
