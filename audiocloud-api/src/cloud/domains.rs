//! Cloud APIs for Domains

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::cloud::apps::Maintenance;
use crate::model::{Model, ResourceId};
use crate::newtypes::{AppId, AppSessionId, DomainId, FixedInstanceId, ModelId};
use crate::session::Session;

/// Used by domain when it is booting
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BootDomain {
    pub domain_id:          DomainId,
    pub event_base:         u64,
    pub fixed_instances:    HashMap<FixedInstanceId, DomainFixedInstance>,
    pub dynamic_instances:  HashMap<ModelId, DynamicInstanceLimits>,
    pub sessions:           HashMap<AppSessionId, Session>,
    pub models:             HashMap<ModelId, Model>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DynamicInstanceLimits {
    pub max_instances: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DomainLimits {
    pub max_sessions: usize,
    pub resources:    HashMap<ResourceId, f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DomainPowerInstanceSettings {
    pub warm_up_ms:        usize,
    pub cool_down_ms:      usize,
    pub idle_off_delay_ms: usize,
    pub instance:          FixedInstanceId,
    pub channel:           usize,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DomainMediaInstanceSettings {
    pub length:          f64,
    pub rewind_to_start: bool,
}
