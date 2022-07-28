use std::collections::{HashMap, HashSet};

use derive_more::{Constructor, Deref, DerefMut, Display, From, IsVariant, Unwrap};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::newtypes::{FilterId, ParameterId, ReportId};
use crate::time::Timestamped;

#[derive(Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Debug, IsVariant)]
pub enum ModelValueUnit {
    #[serde(rename = "no")]
    Unitless,
    #[serde(rename = "percent")]
    Percent,
    #[serde(rename = "dB")]
    Decibels,
    #[serde(rename = "hz")]
    Hertz,
    #[serde(rename = "oct")]
    Octaves,
    #[serde(rename = "toggle")]
    Toggle,
    #[serde(rename = "amps")]
    Amperes,
    #[serde(rename = "watthrs")]
    WattHours,
}

impl Default for ModelValueUnit {
    fn default() -> Self {
        Self::Unitless
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, IsVariant, Unwrap)]
#[serde(untagged)]
pub enum ModelValueOption {
    Single(ModelValue),
    Range(ModelValue, ModelValue),
}

impl ModelValueOption {
    pub fn num_range(min: f64, max: f64) -> Self {
        Self::Range(ModelValue::Number(min), ModelValue::Number(max))
    }

    pub fn zero_to(max: f64) -> Self {
        Self::num_range(0f64, max)
    }

    pub fn to_zero(min: f64) -> Self {
        Self::num_range(min, 0f64)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, IsVariant, Unwrap)]
#[serde(untagged)]
pub enum ModelValue {
    String(String),
    Number(f64),
    Bool(bool),
}

impl ModelValue {
    pub fn to_f64(&self) -> Option<f64> {
        match self {
            ModelValue::String(_) => None,
            ModelValue::Number(v) => Some(*v),
            ModelValue::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
        }
    }

    pub fn to_bool(&self) -> Option<bool> {
        match self {
            ModelValue::String(_) => None,
            ModelValue::Number(v) => Some({
                if *v == 0.0 {
                    false
                } else {
                    true
                }
            }),
            ModelValue::Bool(b) => Some(*b),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, IsVariant)]
#[serde(rename_all = "snake_case")]
pub enum ModelInput {
    Audio(ControlChannels),
    Sidechain,
    Midi,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, IsVariant)]
#[serde(rename_all = "snake_case")]
pub enum ModelOutput {
    Audio(ControlChannels),
    Midi,
}

pub type ModelInputs = Vec<ModelInput>;
pub type ModelOutputs = Vec<ModelOutput>;

pub type ModelParameters = HashMap<ParameterId, ModelParameter>;
pub type ModelReports = HashMap<ReportId, ModelReport>;

/// A model describes the parameters and reprots of a processor
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Model {
    #[serde(default)]
    pub resources:    HashMap<ResourceId, f64>,
    pub inputs:       ModelInputs,
    pub outputs:      ModelOutputs,
    pub parameters:   ModelParameters,
    pub reports:      ModelReports,
    pub media:        bool,
    #[serde(default)]
    pub capabilities: HashSet<ModelCapability>,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum ModelCapability {
    PowerDistributor,
    AudioRouter,
    AudioMixer,
    DigitalInputOutput,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant, Unwrap)]
#[serde(rename_all = "snake_case")]
pub enum ModelParameterRole {
    #[unwrap(ignore)]
    NoRole,
    Power,
    Global(GlobalParameterRole),
    Channel(ChannelParameterRole),
    Amplifier(AmplifierId, AmplifierParameterRole),
    Dynamics(DynamicsId, DynamicsParameterRole),
    Filter(FilterId, FilterParameterRole),
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant)]
#[serde(rename_all = "snake_case")]
pub enum ChannelParameterRole {
    Pan,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant)]
#[serde(rename_all = "snake_case")]
pub enum GlobalParameterRole {
    Enable,
    Bypass,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant)]
#[serde(rename_all = "snake_case")]
pub enum AmplifierParameterRole {
    Enable,
    Gain,
    Distortion,
    SlewRate,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant)]
#[serde(rename_all = "snake_case")]
pub enum DynamicsParameterRole {
    Ratio,
    Threshold,
    Ceiling,
    Attack,
    Release,
    AutoRelease,
    AutoAttack,
    AutoRatio,
    Knee,
    DetectorInput,
    DetectorMaterial,
    DetectorFilter,
    MidEmphasis,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant)]
#[serde(rename_all = "snake_case")]
pub enum FilterParameterRole {
    Gain,
    GainDirection,
    Frequency,
    Bandwidth,
    Type,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant, Unwrap)]
#[serde(rename_all = "snake_case")]
pub enum ModelReportRole {
    #[unwrap(ignore)]
    NoRole,
    Power(PowerReportRole),
    Amplifier(AmplifierId, AmplifierReportRole),
    Dynamics(DynamicsId, DynamicsReportRole),
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant)]
#[serde(rename_all = "snake_case")]
pub enum PowerReportRole {
    Powered,
    Current,
    PowerFactor,
    TotalEnergy,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant)]
#[serde(rename_all = "snake_case")]
pub enum AmplifierReportRole {
    PeakVolume,
    RmsVolume,
    LufsVolumeMomentary,
    LufsVolumeShortTerm,
    LufsVolumeIntegrated,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant)]
#[serde(rename_all = "snake_case")]
pub enum DynamicsReportRole {
    GainReduction,
    GainReductionLimitHit,
}

#[serde_as]
#[derive(Serialize, Deserialize, Deref, DerefMut, Debug, Clone, PartialEq, From, Constructor)]
pub struct MultiChannelValue(#[serde_as(as = "Vec<(_, _)>")] HashMap<usize, ModelValue>);

#[serde_as]
#[derive(Serialize, Deserialize, Deref, DerefMut, Debug, Clone, PartialEq, From, Constructor)]
pub struct MultiChannelTimestampedValue(#[serde_as(as = "Vec<(_, _)>")] HashMap<usize, Timestamped<ModelValue>>);

pub mod multi_channel_value {
    use maplit::hashmap;

    use crate::model::{ModelValue, MultiChannelValue};

    pub fn bool(channel: usize, value: bool) -> MultiChannelValue {
        (hashmap! {
            channel => ModelValue::Bool(value),
        }).into()
    }

    pub fn number(channel: usize, value: f64) -> MultiChannelValue {
        (hashmap! {
            channel => ModelValue::Number(value),
        }).into()
    }

    pub fn string(channel: usize, value: String) -> MultiChannelValue {
        (hashmap! {
            channel => ModelValue::String(value),
        }).into()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ModelParameter {
    pub scope:  ModelElementScope,
    #[serde(default)]
    pub unit:   ModelValueUnit,
    pub role:   ModelParameterRole,
    pub values: Vec<ModelValueOption>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ModelElementScope {
    Global,
    AllInputs,
    AllOutputs,
    Size(usize),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ModelReport {
    pub scope:  ModelElementScope,
    #[serde(default)]
    pub unit:   ModelValueUnit,
    pub role:   ModelReportRole,
    pub values: Vec<ModelValueOption>,
    #[serde(default)]
    pub public: bool,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, IsVariant)]
#[serde(rename_all = "snake_case")]
pub enum ControlChannels {
    Global,
    Left,
    Right,
    Generic,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Display)]
pub enum ResourceId {
    // in GiB
    #[serde(rename = "ram")]
    Memory,
    // in Ghz
    #[serde(rename = "cpu")]
    CPU,
    // in cuda cores
    #[serde(rename = "gpu")]
    GPU,
    // in percent?
    #[serde(rename = "antelope_dsp")]
    AntelopeDSP,
    // in percent?
    #[serde(rename = "universal_audio_dsp")]
    UniversalAudioDSP,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant)]
#[serde(rename_all = "snake_case")]
pub enum AmplifierId {
    Input,
    Output,
    Global,
    InsertInput,
    InsertOutput,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant)]
#[serde(rename_all = "snake_case")]
pub enum DynamicsId {
    Total,
    Compressor,
    Gate,
    Limiter,
    DeEsser,
}
