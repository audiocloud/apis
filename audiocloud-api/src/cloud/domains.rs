//! Cloud APIs for Domains

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

use crate::common::model::{Model, ResourceId};
use crate::common::task::Task;
use crate::newtypes::{AppId, AppTaskId, DomainId, FixedInstanceId, ModelId};
use crate::time::{TimeRange, Timestamp};

/// Used by domain when it is booting
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct BootDomainResponse {
    pub domain_id:          DomainId,
    pub event_base:         u64,
    pub fixed_instances:    HashMap<FixedInstanceId, DomainFixedInstance>,
    pub dynamic_instances:  HashMap<ModelId, DynamicInstanceLimits>,
    pub models:             HashMap<ModelId, Model>,
    pub tasks:              HashMap<AppTaskId, Task>,
    pub maintenance:        Vec<Maintenance>,
    pub domain_limits:      DomainLimits,
    pub min_session_len:    f64,
    pub native_sample_rate: usize,
    pub public_url:         String,
    pub cmd_topic:          String,
    pub evt_topic:          String,
    pub kafka_url:          String,
    pub consume_username:   String,
    pub consume_password:   String,
    pub produce_username:   String,
    pub produce_password:   String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct DynamicInstanceLimits {
    pub max_instances: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct DomainLimits {
    pub max_sessions: usize,
    pub resources:    HashMap<ResourceId, f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DomainFixedInstance {
    pub input_start:  Option<u32>,
    pub output_start: Option<u32>,
    pub sidecars:     HashSet<ModelId>,
    pub power:        Option<DomainPowerInstanceSettings>,
    pub media:        Option<DomainMediaInstanceSettings>,
    pub apps:         Vec<AppId>,
    pub maintenance:  Vec<Maintenance>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct InstanceRouting {
    pub send_count:     usize,
    pub send_channel:   usize,
    pub return_count:   usize,
    pub return_channel: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DomainPowerInstanceSettings {
    pub warm_up_ms:        usize,
    pub cool_down_ms:      usize,
    pub idle_off_delay_ms: usize,
    pub instance:          FixedInstanceId,
    pub channel:           usize,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DomainMediaInstanceSettings {
    pub length:          f64,
    pub rewind_to_start: bool,
}

/// Used by APIs for Apps
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetDomainResponse {
    pub fixed_instances:    HashMap<FixedInstanceId, AppFixedInstance>,
    pub dynamic_instances:  HashMap<ModelId, DynamicInstanceLimits>,
    pub domain_limits:      DomainLimits,
    pub min_session_len:    f64,
    pub public_url:         String,
    pub native_sample_rate: usize,
    pub maintenance:        Vec<Maintenance>,
    pub enabled:            bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
pub struct Maintenance {
    pub time:   TimeRange,
    pub reason: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AppFixedInstance {
    pub power:       bool,
    pub media:       bool,
    pub sidecars:    HashSet<ModelId>,
    pub maintenance: Vec<Maintenance>,
}

impl From<DomainFixedInstance> for AppFixedInstance {
    fn from(instance: DomainFixedInstance) -> Self {
        let DomainFixedInstance { sidecars,
                                  power,
                                  media,
                                  maintenance,
                                  .. } = instance;
        Self { power: power.is_some(),
               media: media.is_some(),
               maintenance,
               sidecars }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AddMaintenance {
    pub time:   TimeRange,
    pub reason: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct DeleteMaintenance {
    pub before: Option<Timestamp>,
    pub after:  Option<Timestamp>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DomainUpdated {
    Updated(DomainId),
}

#[utoipa::path(
  get,
  path = "/v1/domains/{domain_id}",
  responses(
    (status = 200, description = "Success", body = GetDomainResponse),
    (status = 401, description = "Not authorized", body = CloudError),
    (status = 404, description = "Not found", body = CloudError),
  ),
  params(
    ("domain_id" = DomainId, Path, description = "Domain to get")
  ))]
pub(crate) fn get() {}

#[utoipa::path(
  get,
  path = "/v1/domains/{domain_id}/boot",
  responses(
    (status = 200, description = "Success", body = BootDomainResponse),
    (status = 401, description = "Not authorized", body = CloudError),
    (status = 404, description = "Not found", body = CloudError),
  ),
  params(
    ("domain_id" = DomainId, Path, description = "Domain to get")
  ))]
pub(crate) fn boot() {}

#[utoipa::path(
  post,
  path = "/v1/domains/{domain_id}/maintenance",
  responses(
    (status = 200, description = "Success", body = DomainUpdated),
    (status = 401, description = "Not authorized", body = CloudError),
    (status = 404, description = "Not found", body = CloudError),
  ),
  params(
    ("domain_id" = DomainId, Path, description = "Domain to get"),
    ("maintenance" = AddMaintenance, description = "Maintenance to add")
  ))]
pub(crate) fn add_maintenance() {}

#[utoipa::path(
  delete,
  path = "/v1/domains/{domain_id}/maintenance",
  responses(
    (status = 200, description = "Success", body = DomainUpdated),
    (status = 401, description = "Not authorized", body = CloudError),
    (status = 404, description = "Not found", body = CloudError),
  ),
  params(
    ("domain_id" = DomainId, Path, description = "Domain to get"),
    ("delete" = DeleteMaintenance, description = "Delete maintenances in this range")
  ))]
pub(crate) fn delete_maintenance() {}

#[utoipa::path(
  post,
  path = "/v1/domains/{domain_id}/instances/{manufacturer}/{name}/{instance}/maintenance",
  responses(
    (status = 200, description = "Success", body = DomainUpdated),
    (status = 401, description = "Not authorized", body = CloudError),
    (status = 404, description = "Not found", body = CloudError),
  ),
  params(
    ("domain_id" = DomainId, Path, description = "Domain to get"),
    ("manufacturer" = String, Path, description = "Manufacture"),
    ("name" = String, Path, description = "Name"),
    ("instance" = String, Path, description = "Instance"),
    ("maintenance" = AddMaintenance, description = "Maintenance to add")
  ))]
pub(crate) fn add_fixed_instance_maintenance() {}

#[utoipa::path(
  delete,
  path = "/v1/domains/{domain_id}/instances/{manufacturer}/{name}/{instance}/maintenance",
  responses(
    (status = 200, description = "Success", body = DomainUpdated),
    (status = 401, description = "Not authorized", body = CloudError),
    (status = 404, description = "Not found", body = CloudError),
  ),
  params(
    ("domain_id" = DomainId, Path, description = "Domain to get"),
    ("manufacturer" = String, Path, description = "Manufacture"),
    ("name" = String, Path, description = "Name"),
    ("instance" = String, Path, description = "Instance"),
    ("delete" = DeleteMaintenance, description = "Delete maintenances in this range")
  ))]
pub(crate) fn delete_fixed_instance_maintenance() {}
