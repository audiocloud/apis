//! API definitions for communicating with the apps
use chrono::Utc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::common::change::{DesiredTaskPlayState, TaskPlayState};
use crate::common::media::{PlayId, RenderId};
use crate::common::time::Timestamp;
use crate::domain::tasks::TaskUpdated;
use crate::domain::DomainError;
use crate::{AppTaskId, ModifyTaskSpec, RequestId, SecureKey, SerializableResult, SocketId, TaskEvent};

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct StreamStats {
    pub id:    AppTaskId,
    pub state: TaskPlayState,
    pub play:  PlayId,
    pub low:   u64,
    pub high:  u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SessionPacketError {
    Playing(PlayId, String),
    Rendering(RenderId, String),
    General(String),
}

/// Difference stamped in milliseconds since a common epoch, in order to pack most efficiently
/// The epoch in InstancePacket is the created_at field of SessionPacket
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
pub struct DiffStamped<T>(usize, T);

impl<T> DiffStamped<T> {
    pub fn new(timestamp: Timestamp, value: T) -> Self {
        (timestamp, value).into()
    }
}

impl<T> From<(Timestamp, T)> for DiffStamped<T> {
    fn from(value: (Timestamp, T)) -> Self {
        let (timestamp, value) = value;
        let diff = Utc::now() - timestamp;
        Self(diff.num_milliseconds() as usize, value)
    }
}

/// A mesasge received over a real-time communication channel from a streaming domain connection
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum SocketMessage {
    /// Task generated event
    TaskEvent {
        /// Id of the task generating the event
        task_id: AppTaskId,
        /// Event details
        event:   TaskEvent,
    },
    /// Response to a request to change a task play state
    SetDesiredPlayStateResponse {
        /// Request id this message is responding to
        request_id: RequestId,
        /// Result
        result:     SerializableResult<TaskUpdated, DomainError>,
    },
    /// Response to a request to change task specification
    ModifyTaskSpecResponse {
        /// Request id this message is responding to
        request_id: RequestId,
        /// Result of the operation
        result:     SerializableResult<TaskUpdated, DomainError>,
    },
    /// Response to initiating a new peer connection
    PeerConnectionResponse {
        /// Request id this message is responding to
        request_id: RequestId,
        /// Result of the operation - the assigned socket ID
        result:     SerializableResult<SocketId, DomainError>,
    },
    /// Response to submitting a peer connection candidate
    PeerConnectionCandidateResponse {
        /// Request id this message is responding to
        request_id: RequestId,
        /// Result of the operation
        result:     SerializableResult<(), DomainError>,
    },
    /// Response to a request to attach the socket to a task
    AttachToTaskResponse {
        /// Request id this message is responding to
        request_id: RequestId,
        /// Result of the operation
        result:     SerializableResult<(), DomainError>,
    },
    /// Response to detach the socket from a task
    DetachFromTaskResponse {
        /// Request id this message is responding to
        request_id: RequestId,
        /// Result of the operation - will be success even if task does not exist
        result:     SerializableResult<(), DomainError>,
    },
    /// Submit a new WebRTC peer connection ICE candidate
    SubmitPeerConnectionCandidate {
        /// Request id (to reference the response to)
        request_id: RequestId,
        /// Socket id of the peer connection
        socket_id:  SocketId,
        /// ICE Candidate
        candidate:  serde_json::Value,
    },
}

/// A message sent over a real-time communication channel to a streaming domain connection
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum SocketRequestMessage {
    /// Request desired task play state
    RequestSetDesiredPlayState {
        /// Request id (to reference the response to)
        request_id: RequestId,
        /// Id of the task to change play state
        task_id:    AppTaskId,
        /// Desired play state
        desired:    DesiredTaskPlayState,
    },
    /// Request to modify task specification
    RequestModifyTaskSpec {
        /// Request id (to reference the response to)
        request_id:   RequestId,
        /// Id of the task to modify
        task_id:      AppTaskId,
        /// List of modifications to apply
        modification: Vec<ModifyTaskSpec>,
    },
    /// Request a new WebRTC peer connection to the domain
    RequestPeerConnection {
        /// Request id (to reference the response to)
        request_id:  RequestId,
        /// Socket id of the peer connection
        socket_id:   SocketId,
        /// Local description offer
        description: serde_json::Value,
    },
    /// Submit a new WebRTC peer connection ICE candidate
    SubmitPeerConnectionCandidate {
        /// Request id (to reference the response to)
        request_id: RequestId,
        /// Socket id of the peer connection
        socket_id:  SocketId,
        /// ICE Candidate
        candidate:  serde_json::Value,
    },
    /// Request attaching to a task
    RequestAttachToTask {
        /// Request id (to reference the response to)
        request_id: RequestId,
        /// Id of the task to attach to
        task_id:    AppTaskId,
        /// Secure key to use for attachment
        secure_key: SecureKey,
    },
    RequestDetachFromTask {
        /// Request id (to reference the response to)
        request_id: RequestId,
        /// Id of the task to attach to
        task_id:    AppTaskId,
    },
}

/// Load packet data
///
/// For each PlayId, on a task, a stream is kept in memory with a history of packets, by ascending
/// serial number. For a sane amount of time, the packets may be requested by the clients. If a
/// packet is not yet models (but it is expected they will be, in the future) the request will
/// block (wait) for `Timeout` milliseconds before giving up and returning 408.
#[utoipa::path(
  get,
  path = "/v1/stream/{app_id}/{task_id}/{play_id}/packet/{serial}",
  responses(
    (status = 200, description = "Success", body = StreamingPacket),
    (status = 401, description = "Not authorized", body = DomainError),
    (status = 404, description = "App, task or stream not found", body = DomainError),
    (status = 408, description = "Timed out waiting for packet", body = DomainError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id"),
    ("play_id" = PlayId, Path, description = "Play id"),
    ("serial" = u64, Path, description = "Packet serial number"),
    ("Timeout" = u64, Header, description = "Milliseconds to wait for the packet to be ready")
  ))]
pub(crate) fn stream_packets() {}

/// Get stream statistics
///
/// Get statistics about cached packets available in the stream.
#[utoipa::path(
  get,
  path = "/v1/stream/{app_id}/{task_id}/{play_id}",
  responses(
    (status = 200, description = "Success", body = StreamStats),
    (status = 401, description = "Not authorized", body = DomainError),
    (status = 404, description = "Not found", body = DomainError),
  ),
  params(
    ("app_id" = AppId, Path, description = "App id"),
    ("task_id" = TaskId, Path, description = "Task id"),
    ("play_id" = PlayId, Path, description = "Play id")
  ))]
pub(crate) fn stream_stats() {}
