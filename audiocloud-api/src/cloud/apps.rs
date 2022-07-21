//! Cloud APIs for apps

use std::collections::{HashMap, HashSet};

use crate::cloud::CloudError;
use serde::{Deserialize, Serialize};

use crate::cloud::domains::DomainFixedInstance;
use crate::cloud::domains::{DomainLimits, DynamicInstanceLimits};
use crate::newtypes::{DomainId, DynamicId, FixedId, FixedInstanceId, InputId, MixerId, ModelId, SecureKey, SessionId, TrackId};
use crate::session::{
    MixerInput, SessionDynamicInstance, SessionFixedInstance, SessionMixer, SessionObjectId, SessionSecurity, SessionTrack,
};
use crate::time::TimeRange;

/// Used by APIs for Apps
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppDomain {
    pub fixed_instances:    HashMap<FixedInstanceId, AppFixedInstance>,
    pub dynamic_instances:  HashMap<ModelId, DynamicInstanceLimits>,
    pub domain_limits:      DomainLimits,
    pub min_session_len:    f64,
    pub public_url:         String,
    pub native_sample_rate: usize,
    pub maintenance:        Vec<Maintenance>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Maintenance {
    pub time:   TimeRange,
    pub reason: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateSession {
    pub domain:   DomainId,
    pub time:     TimeRange,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SessionSpec {
    #[serde(default)]
    pub tracks:  HashMap<TrackId, SessionTrack>,
    #[serde(default)]
    pub mixers:  HashMap<MixerId, SessionMixer>,
    #[serde(default)]
    pub dynamic: HashMap<DynamicId, SessionDynamicInstance>,
    #[serde(default)]
    pub fixed:   HashMap<FixedId, SessionFixedInstance>,
}

impl SessionSpec {
    pub fn validate(&self) -> Result<(), CloudError> {
        if self.fixed.is_empty() && self.dynamic.is_empty() && self.mixers.is_empty() && self.tracks.is_empty() {
            return Err(CloudError::InternalInconsistency(format!("No tracks, mixers, dynamic instances, or fixed instances declared in session spec")));
        }

        for (mixer_id, mixer) in &self.mixers {
            let session_object_id = SessionObjectId::Mixer(mixer_id.clone());
            for (input_id, input) in &mixer.inputs {
                self.check_input(&session_object_id, input_id, input)?;
            }
        }

        Ok(())
    }

    pub fn fixed_instance_to_fixed_id(&self, instance_id: &FixedInstanceId) -> Option<&FixedId> {
        for (fixed_id, fixed) in &self.fixed {
            if &fixed.instance_id == instance_id {
                return Some(fixed_id);
            }
        }
        None
    }

    fn check_input(&self, session_object_id: &SessionObjectId, input_id: &InputId, input: &MixerInput) -> Result<(), CloudError> {
        match &input.source_id {
            SessionObjectId::Track(track_id) => {
                if !self.tracks.contains_key(track_id) {
                    return Err(CloudError::SourceTrackNotFound(session_object_id.clone(),
                                                               input_id.clone(),
                                                               track_id.clone()));
                }
            }
            SessionObjectId::Mixer(mixer_id) => {
                if !self.mixers.contains_key(mixer_id) {
                    return Err(CloudError::SourceMixerNotFound(session_object_id.clone(),
                                                               input_id.clone(),
                                                               mixer_id.clone()));
                }
            }
            SessionObjectId::DynamicInstance(dynamic_id) => {
                if !self.dynamic.contains_key(dynamic_id) {
                    return Err(CloudError::SourceDynamicInstanceNotFound(session_object_id.clone(),
                                                                         input_id.clone(),
                                                                         dynamic_id.clone()));
                }
            }
            SessionObjectId::FixedInstance(fixed_id) => {
                if !self.fixed.contains_key(fixed_id) {
                    return Err(CloudError::SourceFixedInstanceNotFound(session_object_id.clone(),
                                                                       input_id.clone(),
                                                                       fixed_id.clone()));
                }
            }
        }

        Ok(())
    }
}
