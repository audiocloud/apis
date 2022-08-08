//! Cloud APIs for apps

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use CloudError::*;

use crate::cloud::domains::DomainFixedInstance;
use crate::cloud::domains::{DomainLimits, DynamicInstanceLimits};
use crate::cloud::CloudError;
use crate::model::Model;
use crate::newtypes::{ConnectionId, DomainId, DynamicId, FixedId, FixedInstanceId, MixerId, ModelId, SecureKey, TrackId};
use crate::session::{
    MixerChannels, SessionConnection, SessionDynamicInstance, SessionFixedInstance, SessionFlowId, SessionMixer, SessionSecurity,
    SessionTrack,
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
    pub domain:      DomainId,
    pub time:        TimeRange,
    #[serde(default)]
    pub tracks:      HashMap<TrackId, SessionTrack>,
    #[serde(default)]
    pub mixers:      HashMap<MixerId, SessionMixer>,
    #[serde(default)]
    pub dynamic:     HashMap<DynamicId, SessionDynamicInstance>,
    #[serde(default)]
    pub fixed:       HashMap<FixedId, SessionFixedInstance>,
    #[serde(default)]
    pub connections: HashMap<ConnectionId, SessionConnection>,
    #[serde(default)]
    pub security:    HashMap<SecureKey, SessionSecurity>,
    pub dry_run:     bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct SessionSpec {
    #[serde(default)]
    pub tracks:      HashMap<TrackId, SessionTrack>,
    #[serde(default)]
    pub mixers:      HashMap<MixerId, SessionMixer>,
    #[serde(default)]
    pub dynamic:     HashMap<DynamicId, SessionDynamicInstance>,
    #[serde(default)]
    pub fixed:       HashMap<FixedId, SessionFixedInstance>,
    #[serde(default)]
    pub connections: HashMap<ConnectionId, SessionConnection>,
}

impl SessionSpec {
    pub fn validate(&self, models: &HashMap<ModelId, Model>) -> Result<(), CloudError> {
        if self.fixed.is_empty() && self.dynamic.is_empty() && self.mixers.is_empty() && self.tracks.is_empty() {
            return Err(InternalInconsistency(format!("No tracks, mixers, dynamic instances, or fixed instances declared in session spec")));
        }

        for (connection_id, connection) in self.connections.iter() {
            self.validate_connection(connection_id, connection, models)?;
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

    fn validate_connection(&self,
                           id: &ConnectionId,
                           connection: &SessionConnection,
                           models: &HashMap<ModelId, Model>)
                           -> Result<(), CloudError> {
        let to = &connection.to;
        let from = &connection.from;

        if !from.is_output() {
            return Err(InternalInconsistency(format!("Connection {id} flow from {from} is not an output")));
        }

        if !to.is_input() {
            return Err(InternalInconsistency(format!("Connection {id} flow to {to} is not an input")));
        }

        self.check_channel_exists(id, &connection.from, &connection.from_channels, models)?;
        self.check_channel_exists(id, &connection.to, &connection.to_channels, models)?;

        Ok(())
    }

    fn check_channel_exists(&self,
                            id: &ConnectionId,
                            flow_id: &SessionFlowId,
                            channels: &MixerChannels,
                            models: &HashMap<ModelId, Model>)
                            -> Result<(), CloudError> {
        match flow_id {
            SessionFlowId::MixerInput(mixer_id) => self.check_channel_exists_mixer(id, mixer_id, channels),
            SessionFlowId::MixerOutput(mixer_id) => self.check_channel_exists_mixer(id, mixer_id, channels),
            SessionFlowId::FixedInstanceInput(fixed_id) => self.check_channel_exists_fixed(id, fixed_id, channels, false, models),
            SessionFlowId::FixedInstanceOutput(fixed_id) => self.check_channel_exists_fixed(id, fixed_id, channels, true, models),
            SessionFlowId::DynamicInstanceInput(dynamic_id) => self.check_channel_exists_dynamic(id, dynamic_id, channels, false, models),
            SessionFlowId::DynamicInstanceOutput(dynamic_id) => self.check_channel_exists_dynamic(id, dynamic_id, channels, true, models),
            SessionFlowId::TrackOutput(track_id) => self.check_channel_exists_track(id, track_id, channels),
        }
    }

    fn check_channel_exists_mixer(&self, id: &ConnectionId, mixer_id: &MixerId, channels: &MixerChannels) -> Result<(), CloudError> {
        let mixer = self.mixers
                        .get(mixer_id)
                        .ok_or_else(|| InternalInconsistency(format!("Connection {id} flow to mixer {mixer_id} does not exist")))?;

        if !channels.is_subset_of(0..mixer.channels) {
            return Err(InternalInconsistency(format!("Connection {id} flow to mixer {mixer_id} has channels that do not exist")));
        }

        Ok(())
    }

    fn check_channel_exists_fixed(&self,
                                  id: &ConnectionId,
                                  fixed_id: &FixedId,
                                  channels: &MixerChannels,
                                  output: bool,
                                  models: &HashMap<ModelId, Model>)
                                  -> Result<(), CloudError> {
        let fixed = self.fixed
                        .get(fixed_id)
                        .ok_or_else(|| InternalInconsistency(format!("Connection {id} references fixed {fixed_id} which does not exist")))?;

        let model_id = fixed.instance_id.model_id();
        let model = models.get(&model_id).ok_or_else(|| {
            InternalInconsistency(format!("Connection {id} references fixed instance labelled {fixed_id} which references model {model_id} which does not exist"))
        })?;

        if !channels.is_subset_of(0..(if output { model.outputs.len() } else { model.inputs.len() })) {
            return Err(InternalInconsistency(format!("Connection {id} references fixed instance labelled {fixed_id} which has channels that do not exist")));
        }

        Ok(())
    }

    fn check_channel_exists_dynamic(&self,
                                    id: &ConnectionId,
                                    dynamic_id: &DynamicId,
                                    channels: &MixerChannels,
                                    output: bool,
                                    models: &HashMap<ModelId, Model>)
                                    -> Result<(), CloudError> {
        let dynamic = self.dynamic.get(dynamic_id).ok_or_else(|| {
            InternalInconsistency(format!("Connection {id} references dynamic instance labelled {dynamic_id} which does not exist"))
        })?;

        let model_id = &dynamic.model_id;
        let model = models.get(&model_id).ok_or_else(|| {
            InternalInconsistency(format!("Connection {id} references dynamic instance labelled {dynamic_id} which references model {model_id} which does not exist"))
        })?;

        if !channels.is_subset_of(0..(if output { model.outputs.len() } else { model.inputs.len() })) {
            return Err(InternalInconsistency(format!("Connection {id} references dynamic instance labelled {dynamic_id} which has channels that do not exist")));
        }

        Ok(())
    }

    fn check_channel_exists_track(&self, id: &ConnectionId, track_id: &TrackId, channels: &MixerChannels) -> Result<(), CloudError> {
        let track = self.tracks
                        .get(track_id)
                        .ok_or_else(|| InternalInconsistency(format!("Connection {id} references track {track_id} which does not exist")))?;

        if !channels.is_subset_of(0..track.channels.num_channels()) {
            return Err(InternalInconsistency(format!("Connection {id} references track {track_id} which has channels that do not exist")));
        }

        Ok(())
    }
}
