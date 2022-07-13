use std::collections::HashMap;

use derive_more::{Display, IsVariant, Unwrap};
use serde::{Deserialize, Serialize};

use crate::newtypes::{FilterId, ParameterId, ReportId};

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
    pub fn to_f64(self) -> Option<f64> {
        match self {
            ModelValue::String(_) => None,
            ModelValue::Number(v) => Some(v),
            ModelValue::Bool(b) => Some(if b { 1.0 } else { 0.0 }),
        }
    }
}

pub type ModelInputs = Vec<(ControlChannels, InputChannelRole)>;
pub type ModelOutputs = Vec<(ControlChannels, OutputChannelRole)>;

pub type ModelParameters = HashMap<ParameterId, ModelParameter>;
pub type ModelReports = HashMap<ReportId, ModelReport>;

/// A model describes the parameters and reprots of a processor
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Model {
    #[serde(default)]
    pub resources: HashMap<ResourceId, f64>,
    pub inputs: ModelInputs,
    pub outputs: ModelOutputs,
    pub parameters: ModelParameters,
    pub reports: ModelReports,
    pub media: bool,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum InputChannelRole {
    Audio,
    SideChain,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum OutputChannelRole {
    Audio,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant, Unwrap)]
#[serde(rename_all = "snake_case")]
pub enum ModelParameterRole {
    #[unwrap(ignore)]
    NoRole,
    Global(GlobalParameterRole),
    Amplifier(AmplifierId, AmplifierParameterRole),
    Dynamics(DynamicsId, DynamicsParameterRole),
    Filter(FilterId, FilterParameterRole),
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
    Amplifier(AmplifierId, AmplifierReportRole),
    Dynamics(DynamicsId, DynamicsReportRole),
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

pub type MultiChannelValue = Vec<(usize, ModelValue)>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ModelParameter {
    pub scope: ModelElementScope,
    #[serde(default)]
    pub unit: ModelValueUnit,
    pub role: ModelParameterRole,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    pub scope: ModelElementScope,
    #[serde(default)]
    pub unit: ModelValueUnit,
    pub role: ModelReportRole,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<ModelValueOption>,
    #[serde(default)]
    pub public: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, IsVariant)]
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
