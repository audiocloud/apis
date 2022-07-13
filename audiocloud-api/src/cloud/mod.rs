//! API definitions for the Cloud

use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::change::ModifySessionError;
use crate::model::ResourceId;
use crate::newtypes::DomainId;
use crate::newtypes::{
    AppId, DynamicId, FixedId, FixedInstanceId, InputId, MixerId, ModelId, SecureKey, SessionId, TrackId,
};
use crate::session::{SessionDynamicInstance, SessionFixedInstance, SessionMixer, SessionTrack};
use crate::time::TimeRange;

pub mod apps;
pub mod domains;
pub mod media;

#[derive(Serialize, Deserialize, Debug, Clone, Error)]
pub enum CloudError {
    #[error("At least a segment of a reservation needs to be in the future")]
    OnlyFutureReservations,

    #[error("Session time must be well-formed")]
    TimeMalformed,

    #[error("Session duration is smaller than domain minimum session time {0} ms")]
    DurationTooShort(f64),

    #[error("Too many sessions reserved on domain")]
    TooManySessions,

    #[error("Detected internal inconsistency: {0}")]
    InternalInconsistency(String),

    #[error("Instances overlapping: {0:?}")]
    OverlappingFixedInstances(HashSet<FixedInstanceId>),

    #[error("Domain {0} unknown")]
    DomainNotFound(DomainId),

    #[error("Instance {0} unknown")]
    InstanceNotFound(FixedInstanceId),

    #[error("Bus {0} references mixer {2} with input {1}, which was not found")]
    SourceBusNotFound(MixerId, InputId, MixerId),

    #[error("Bus {0} references track {2} with input {1}, which was not found")]
    SourceTrackNotFound(MixerId, InputId, TrackId),

    #[error("Bus {0} references fixed instance {2} with input {1}, which was not found")]
    SourceFixedInstanceNotFound(MixerId, InputId, FixedId),

    #[error("Bus {0} references dynamic instance {2} with input {1}, which was not found")]
    SourceDynamicInstanceNotFound(MixerId, InputId, DynamicId),

    #[error("Instance {1} required by bus {0} but not reserved")]
    InstanceNotReferenced(u64, FixedInstanceId),

    #[error("Model {2} of a dynamic instance required by bus {0} is not supported on domain {1}")]
    DynamicInstanceNotSupported(DynamicId, DomainId, ModelId),

    #[error("Fixed instance {2} required by bus {0} is not supported on domain {1}")]
    FixedInstanceNotSupported(FixedId, DomainId, FixedInstanceId),

    #[error("Out of {0} resource by {1}")]
    OutOfResource(ResourceId, f64),

    #[error("Object source {0} on bus {1} is too short-lived")]
    ObjectTooShortLived(DomainId, u64, u64),

    #[error("Session {0} was not found")]
    SessionNotFound(SessionId),

    #[error("Session could not be modified: {0}")]
    SessionModification(#[from] ModifySessionError),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Authorization failed: {0}")]
    Authorization(String),

    #[error("All retries exhausted while trying to obtain a lock")]
    BlockingLock,
}
