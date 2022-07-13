//! Cloud APIs for apps

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::cloud::domains::DomainFixedInstance;
use crate::cloud::domains::{DomainLimits, DynamicInstanceLimits};
use crate::newtypes::{
    AppId, DomainId, DynamicId, FixedId, FixedInstanceId, MixerId, ModelId, SecureKey, SessionId, TrackId,
};
use crate::session::{SessionDynamicInstance, SessionFixedInstance, SessionMixer, SessionSecurity, SessionTrack};
use crate::time::TimeRange;

/// Used by APIs for Apps
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppDomain {
    pub fixed_instances: HashMap<FixedInstanceId, AppFixedInstance>,
    pub dynamic_instances: HashMap<ModelId, DynamicInstanceLimits>,
    pub domain_limits: DomainLimits,
    pub min_session_len: f64,
    pub public_url: String,
    pub native_sample_rate: usize,
    pub maintenance: Vec<Maintenance>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Maintenance {
    pub time: TimeRange,
    pub reason: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppFixedInstance {
    pub power: bool,
    pub media: bool,
    pub sidecars: HashSet<ModelId>,
    pub maintenance: Vec<Maintenance>,
}

impl From<DomainFixedInstance> for AppFixedInstance {
    fn from(instance: DomainFixedInstance) -> Self {
        let DomainFixedInstance {
            sidecars,
            power,
            media,
            maintenance,
            ..
        } = instance;
        Self {
            power: power.is_some(),
            media: media.is_some(),
            maintenance,
            sidecars,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateSession {
    pub app: AppId,
    pub domain: DomainId,
    pub id: SessionId,
    pub time: TimeRange,
    #[serde(default)]
    pub tracks: HashMap<TrackId, SessionTrack>,
    #[serde(default)]
    pub mixers: HashMap<MixerId, SessionMixer>,
    #[serde(default)]
    pub dynamic: HashMap<DynamicId, SessionDynamicInstance>,
    #[serde(default)]
    pub fixed: HashMap<FixedId, SessionFixedInstance>,
    #[serde(default)]
    pub security: HashMap<SecureKey, SessionSecurity>,
    pub dry_run: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SessionSpec {
    #[serde(default)]
    pub tracks: HashMap<TrackId, SessionTrack>,
    #[serde(default)]
    pub mixers: HashMap<MixerId, SessionMixer>,
    #[serde(default)]
    pub dynamic: HashMap<DynamicId, SessionDynamicInstance>,
    #[serde(default)]
    pub fixed: HashMap<FixedId, SessionFixedInstance>,
}
