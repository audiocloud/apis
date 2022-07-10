//! Cloud APIs for apps

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::cloud::domains::DomainFixedInstance;
use crate::cloud::domains::{DomainLimits, DynamicInstanceLimits};
use crate::newtypes::{AppId, DomainId, DynamicId, FixedId, FixedInstanceId, MixerId, ModelId, SecureKey, TrackId};
use crate::session::{SessionDynamicInstance, SessionFixedInstance, SessionMixer, SessionSecurity, SessionTrack};
use crate::time::TimeRange;

/// Used by APIs for Apps
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppDomain {
  pub fixed_instances:    HashMap<FixedInstanceId, AppFixedInstance>,
  pub dynamic_instances:  HashMap<ModelId, DynamicInstanceLimits>,
  pub domain_limits:      DomainLimits,
  pub min_session_len:    usize,
  pub public_url:         String,
  pub native_sample_rate: usize,
  pub maintenance:        Vec<TimeRange>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppFixedInstance {
  pub power:    bool,
  pub media:    bool,
  pub sidecars: HashSet<ModelId>,
}

impl From<DomainFixedInstance> for AppFixedInstance {
  fn from(instance: DomainFixedInstance) -> Self {
    let DomainFixedInstance { sidecars, power, media, .. } = instance;
    Self { power: power.is_some(),
           media: media.is_some(),
           sidecars }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateOrReplaceSession {
  pub time:     TimeRange,
  pub domain:   DomainId,
  pub app:      AppId,
  #[serde(default)]
  pub tracks:   HashMap<TrackId, SessionTrack>,
  #[serde(default)]
  pub mixers:   HashMap<MixerId, SessionMixer>,
  #[serde(default)]
  pub dynamic:  HashMap<DynamicId, SessionDynamicInstance>,
  #[serde(default)]
  pub fixed:    HashMap<FixedId, SessionFixedInstance>,
  #[serde(default)]
  pub security: HashMap<SecureKey, SessionSecurity>,
  pub dry_run:  bool,
}
