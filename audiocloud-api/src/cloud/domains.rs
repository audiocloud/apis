//! Cloud APIs for Domains

use std::collections::{HashMap, HashSet};

use crate::cloud::apps::Maintenance;
use serde::{Deserialize, Serialize};

use crate::model::{Model, ResourceId};
use crate::newtypes::{DomainId, FixedInstanceId, ModelId, PduId, SessionId};
use crate::session::Session;
use crate::time::TimeRange;

/// Used by domain when it is booting
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BootDomain {
    pub domain_id:          DomainId,
    pub event_base:         u64,
    pub pdus:               HashMap<usize, PduConfig>,
    pub fixed_instances:    HashMap<FixedInstanceId, DomainFixedInstance>,
    pub dynamic_instances:  HashMap<ModelId, DynamicInstanceLimits>,
    pub sessions:           HashMap<SessionId, Session>,
    pub models:             HashMap<ModelId, Model>,
    pub domain_limits:      DomainLimits,
    pub min_session_len:    f64,
    pub native_sample_rate: usize,
    pub public_url:         String,
    pub kafka_url:          String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PduConfig {
    PowerPdu {
        address:  String,
        username: String,
        password: String,
    },
    Mocked,
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
    pub input_start:  u32,
    pub output_start: u32,
    pub sidecars:     HashSet<ModelId>,
    pub power:        Option<DomainPowerInstanceSettings>,
    pub media:        Option<DomainMediaInstanceSettings>,
    pub maintenance:  Vec<Maintenance>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DomainPowerInstanceSettings {
    pub on_delay_ms:       usize,
    pub off_delay_ms:      usize,
    pub idle_off_delay_ms: usize,
    pub pdu_id:            PduId,
    pub channel:           usize,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DomainMediaInstanceSettings {
    pub length:          f64,
    pub rewind_to_start: bool,
}
