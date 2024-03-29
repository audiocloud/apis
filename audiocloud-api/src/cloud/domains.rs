//! Cloud APIs for Domains

use std::collections::{HashMap, HashSet};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::common::model::{Model, ResourceId};
use crate::common::task::Task;
use crate::newtypes::{AppId, AppTaskId, DomainId, FixedInstanceId, ModelId};
use crate::time::{TimeRange, Timestamp};
use crate::EngineId;

/// Used by domain for booting
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DomainConfig {
    /// Id of the domain
    pub domain_id:            DomainId,
    /// Fixed instances configured on the domain
    #[serde(default)]
    pub fixed_instances:      HashMap<FixedInstanceId, DomainFixedInstanceConfig>,
    /// Dynamic instances configured on the domain, with associated limits
    #[serde(default)]
    pub dynamic_instances:    HashMap<ModelId, DynamicInstanceLimits>,
    /// Engines configured on the domain
    #[serde(default)]
    pub engines:              HashMap<EngineId, DomainEngineConfig>,
    /// Currently configured tasks
    #[serde(default)]
    pub tasks:                HashMap<AppTaskId, Task>,
    /// Configured maintenance time windows during which the domain should not serve requests
    #[serde(default)]
    pub maintenance:          Vec<Maintenance>,
    /// Apps allowed to access the domain
    #[serde(default)]
    pub apps:                 HashSet<AppId>,
    /// Maximum number of concurrent tasks (when lower than the sum of tasks available on engines)
    #[serde(default)]
    pub max_concurrent_tasks: Option<usize>,
    /// Minimum Task length
    #[serde(default = "default_min_task_length")]
    pub min_task_len_ms:      i64,
    /// Source for commands from the cloud to the domain
    #[serde(default)]
    pub command_source:       DomainCommandSource,
    /// Sink for events from the domain to the cloud
    #[serde(default)]
    pub event_sink:           DomainEventSink,
    /// Source of model information for the domain (can include unused models)
    pub models:               DomainModelSource,
    /// The public host or IP where domain API is visible to the outside world
    pub public_host:          String,
}

fn default_min_task_length() -> i64 {
    5_000
}

/// Source of commands for domains
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DomainCommandSource {
    /// Domain command source disabled
    Disabled,
    /// Consume a kafka topic
    Kafka {
        /// Topic where commands to the domain will be sent
        topic:    String,
        /// Kafka broker list to be used for commands and events
        brokers:  String,
        /// Username used to consume commands
        username: String,
        /// SASL SCRAM password used to consume commands
        password: String,
        /// Read after this offset from event stream, or default to the latest one persisted
        offset:   Option<i64>,
    },
}

impl Default for DomainCommandSource {
    fn default() -> Self {
        Self::Disabled
    }
}

/// Source of commands for domains
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DomainEventSink {
    /// Disable sending of domain events
    Disabled,
    /// Emit events as logs
    Log,
    /// Produce to a kafka topic
    Kafka {
        /// Topic where events from the domain may be sent
        topic:    String,
        /// Kafka broker list to be used for commands and events
        brokers:  String,
        /// Username used to produce events
        username: String,
        /// SASL SCRAM password used to produce events
        password: String,
    },
}

impl Default for DomainEventSink {
    fn default() -> Self {
        Self::Log
    }
}

/// Source for models
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DomainModelSource {
    /// MOdels are provided in-line with the configuration
    Inline {
        /// All model information for parameter and report validation
        models: HashMap<ModelId, Model>,
    },
    /// Models are stored locally on the filesystem
    Local {
        /// The local path where models are stored
        path: String,
    },
    /// Obtain models from a remote URL
    Remote {
        /// URL where models are going to reside
        url:                 String,
        /// Refresh interval, in milliseconds
        refresh_interval_ms: u64,
    },
}

/// Information about a media engine within a domain
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
pub struct DomainEngineConfig {
    /// Dynamic instances configured on the audio engine, with associated limits
    #[serde(default)]
    pub dynamic_instances:    HashMap<ModelId, DynamicInstanceLimits>,
    /// Maximum number of concurrent tasks
    pub max_concurrent_tasks: usize,
    /// Resources available on the domain
    #[serde(default)]
    pub resources:            HashMap<ResourceId, f64>,
    /// Native audio sample rate
    pub sample_rate:          usize,
}

/// Limits on dynamic instances
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
pub struct DynamicInstanceLimits {
    /// Maximum number of concurrent dynamic instances
    ///
    /// Takes precedence over over total resource usage. For example, there may be more resources
    /// but licensing limits the amount of instances.
    pub max_instances: usize,
}

/// Configuration of a fixed instance
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DomainFixedInstanceConfig {
    /// Engine hosting the instance
    pub engine_id:     EngineId,
    /// Instance inputs start at index on engine
    #[serde(default)]
    pub input_start:   Option<u32>,
    /// Instance outputs start at index on engine
    #[serde(default)]
    pub output_start:  Option<u32>,
    /// Additional models with parameters or reports that are merged with the instance model
    #[serde(default)]
    pub sidecars:      HashSet<ModelId>,
    /// Optional configuration to powers on/off instance to conserve energy
    #[serde(default)]
    pub power:         Option<DomainPowerInstanceConfig>,
    /// Optional configuration if instance handles media (such as tape machines)
    #[serde(default)]
    pub media:         Option<DomainMediaInstanceConfig>,
    /// Apps allowed to access the instance or null if the domain defaults are used
    #[serde(default)]
    pub apps_override: Option<HashSet<AppId>>,
    /// Maintenance windows on this instance
    #[serde(default)]
    pub maintenance:   Vec<Maintenance>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct FixedInstanceRouting {
    pub send_count:     usize,
    pub send_channel:   usize,
    pub return_count:   usize,
    pub return_channel: usize,
}

pub type FixedInstanceRoutingMap = HashMap<FixedInstanceId, FixedInstanceRouting>;

/// Instance power settings
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DomainPowerInstanceConfig {
    /// Number of milliseconds to wait to warm up after powering on
    pub warm_up_ms:        usize,
    /// Number of milliseconds to wait to cool down after powering down
    pub cool_down_ms:      usize,
    /// Number of milliseconds to wait before automatically powering down after idle
    pub idle_off_delay_ms: usize,
    /// Power instance used to distribute power to this instance
    pub instance:          FixedInstanceId,
    /// Which channel on the power instance is distributing power to this instance
    pub channel:           usize,
}

/// Instance media settings
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DomainMediaInstanceConfig {
    /// Lenght of the inserted media in milliseconds
    pub length_ms:               usize,
    /// WHen rewinding to make space for contiguous renders, should the driver rewind to start or just enough to start rendering
    pub renders_rewind_to_start: bool,
    /// Behaviour of playing back (streaming) and hitting end of media
    ///
    /// - If null, rewind to start
    /// - Otherwise, rewind by specified amount of milliseconds
    pub play_rewind:             Option<usize>,
}

/// Domain summary for apps
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct GetDomainResponse {
    /// FIxed instances available on the domain
    pub fixed_instances: HashMap<FixedInstanceId, AppFixedInstance>,
    /// Engines available on the domain
    pub engines:         HashMap<EngineId, DomainEngineConfig>,
    /// Minimum task duration
    pub min_task_len:    f64,
    /// Base public URL for domain API
    pub public_url:      String,
    /// Configured maintenance time windows during which the domain should not serve requests
    pub maintenance:     Vec<Maintenance>,
    /// If true, the domain is enabled and will serve requests if not in maitenance
    pub enabled:         bool,
}

/// Maintenance window
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
pub struct Maintenance {
    /// Time during which maintenance is taking place (may overlap with others)
    pub time:   TimeRange,
    /// Human readable string about it, or URL to a web page detailing more information
    pub reason: String,
}

/// Fixed instance summary for apps
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AppFixedInstance {
    /// If true, the instance may need to be powered up
    pub power:       bool,
    /// If true, the instance is using media and may rewind
    pub media:       bool,
    /// Additional models with parameters or reports that are merged with the instance model
    pub sidecars:    HashSet<ModelId>,
    /// Configured maintenance time windows during which the instance should not serve requests
    pub maintenance: Vec<Maintenance>,
}

impl From<DomainFixedInstanceConfig> for AppFixedInstance {
    fn from(instance: DomainFixedInstanceConfig) -> Self {
        let DomainFixedInstanceConfig { sidecars,
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

/// Add maintenance to an object
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct AddMaintenance {
    /// When is it taking place
    pub time:   TimeRange,
    /// WHat is the reason for maintenance (human readable string or URL with more information
    pub reason: String,
}

/// Clear maintenance from an object
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct ClearMaintenance {
    /// If not null, clear all maitnenance before this timestamp
    pub before: Option<Timestamp>,
    /// If not null, clear all maitnenance after this timestamp
    pub after:  Option<Timestamp>,
}

/// The domain has been updated
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DomainUpdated {
    /// Updated normally
    Updated(DomainId),
}

/// Get domain details
///
/// Get details about a domain. Available to owners, administrators and apps where the app has
/// permission to access domain details.
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
pub(crate) fn get_domain() {}

/// Domain requests to get its configuration
///
/// When a domain starts in cloud mode, it will get the details of its configuration from the cloud.
/// This endpoint delivers all of the cloud information about the domain, including instances,
/// audio engines and cloud synchronization endpoints.
#[utoipa::path(
get,
path = "/v1/domains/{domain_id}/config",
responses(
(status = 200, description = "Success", body = DomainConfig),
(status = 401, description = "Not authorized", body = CloudError),
(status = 404, description = "Not found", body = CloudError),
),
params(
("domain_id" = DomainId, Path, description = "Domain to get config for")
))]
pub(crate) fn get_domain_config() {}

/// Add maitenance time to domain
///
/// Add a designated time of maitnenance to the whole domain. When a domain is in maintenance, it
/// cannot serve API requests or process tasks. Apps will not be able to create bookings against the
/// domain that intersect with maintenance windows.
#[utoipa::path(
post,
path = "/v1/domains/{domain_id}/maintenance",
request_body = AddMaintenance,
responses(
(status = 200, description = "Success", body = DomainUpdated),
(status = 401, description = "Not authorized", body = CloudError),
(status = 404, description = "Not found", body = CloudError),
),
params(
("domain_id" = DomainId, Path, description = "Domain to add maintenance to"),
))]
pub(crate) fn add_domain_maintenance() {}

/// Clear domain maintenance time
///
/// Clear any maitnenance on the domain that matches the time predicates provided.
#[utoipa::path(
delete,
path = "/v1/domains/{domain_id}/maintenance",
request_body = ClearMaintenance,
responses(
(status = 200, description = "Success", body = DomainUpdated),
(status = 401, description = "Not authorized", body = CloudError),
(status = 404, description = "Not found", body = CloudError),
),
params(
("domain_id" = DomainId, Path, description = "Domain to clear maitnenance on"),
))]
pub(crate) fn clear_domain_maintenance() {}

/// Add maitenance time to instance
///
/// Add a designated time of maitnenance to an instance in a domain. When an instance is in
/// maintenance, it cannot process tasks. Apps will not be able to create bookings against the
/// instance that intersect with maintenance windows.
#[utoipa::path(
post,
path = "/v1/domains/{domain_id}/instances/{manufacturer}/{name}/{instance}/maintenance",
request_body = AddMaintenance,
responses(
(status = 200, description = "Success", body = DomainUpdated),
(status = 401, description = "Not authorized", body = CloudError),
(status = 404, description = "Not found", body = CloudError),
),
params(
("domain_id" = DomainId, Path, description = "Domain hosting the instance"),
("manufacturer" = String, Path, description = "Instance manufacturer"),
("name" = String, Path, description = "Instance (product) name"),
("instance" = String, Path, description = "Instance unique identifier"),
))]
pub(crate) fn add_fixed_instance_maintenance() {}

/// Clear instance maintenance time
///
/// Clear any maitnenance on the instance that matches the time predicates provided.
#[utoipa::path(
delete,
path = "/v1/domains/{domain_id}/instances/{manufacturer}/{name}/{instance}/maintenance",
request_body = ClearMaintenance,
responses(
(status = 200, description = "Success", body = DomainUpdated),
(status = 401, description = "Not authorized", body = CloudError),
(status = 404, description = "Not found", body = CloudError),
),
params(
("domain_id" = DomainId, Path, description = "Domain hosting the instance"),
("manufacturer" = String, Path, description = "Instance manufacturer"),
("name" = String, Path, description = "Instance (product) name"),
("instance" = String, Path, description = "Instance unique identifier"),
))]
pub(crate) fn clear_fixed_instance_maintenance() {}
