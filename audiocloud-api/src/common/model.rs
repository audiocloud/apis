use anyhow::anyhow;
use std::collections::{HashMap, HashSet};
use std::iter;

use derive_more::{Display, IsVariant, Unwrap};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::common::time::Timestamped;
use crate::common::{FilterId, ParameterId, ReportId};

#[derive(Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Debug, IsVariant, JsonSchema)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, IsVariant, Unwrap, JsonSchema)]
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

    pub fn get_simple_type(&self) -> anyhow::Result<SimpleModelValueType> {
        match self {
            ModelValueOption::Single(value) => Ok(value.get_simple_type()),
            ModelValueOption::Range(first, second) => {
                if first.is_number() && second.is_number() {
                    Ok(SimpleModelValueType::Number { signed:  true,
                                                      integer: false, })
                } else {
                    Err(anyhow!("Only numeric ranges supported"))
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, IsVariant, Unwrap, JsonSchema)]
#[serde(untagged)]
pub enum ModelValue {
    String(String),
    Number(f64),
    Bool(bool),
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SimpleModelValueType {
    String,
    Number { integer: bool, signed: bool },
    Bool,
}

impl SimpleModelValueType {
    pub fn from_numeric_value(number: f64) -> SimpleModelValueType {
        let mut integer = false;
        let mut signed = false;

        if number.fract() == 0.0 {
            if number.is_sign_negative() {
                signed = true;
            }

            integer = true;
        }

        return Self::Number { integer, signed };
    }

    pub fn try_widen(self, other: SimpleModelValueType) -> anyhow::Result<SimpleModelValueType> {
        match (self, other) {
            (Self::Number { signed: s1, integer: i1 }, Self::Number { signed: s2, integer: i2 }) => Ok(Self::Number { signed:  s1 || s2,
                                                                                                                      integer: i1 && i2, }),
            _ => Err(anyhow!("Only numeric types may be widened")),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, IsVariant, Unwrap, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ModelValueType {
    Single(SimpleModelValueType),
    Either(SimpleModelValueType, SimpleModelValueType),
    Any,
}

impl ModelValue {
    pub fn into_f64(self) -> Option<f64> {
        self.to_f64()
    }

    pub fn to_f64(&self) -> Option<f64> {
        match self {
            ModelValue::String(_) => None,
            ModelValue::Number(v) => Some(*v),
            ModelValue::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
        }
    }

    pub fn into_i64(self) -> Option<i64> {
        self.to_i64()
    }

    pub fn to_i64(&self) -> Option<i64> {
        match self {
            ModelValue::String(_) => None,
            ModelValue::Number(v) => Some(*v as i64),
            ModelValue::Bool(b) => Some(if *b { 1 } else { 0 }),
        }
    }

    pub fn into_bool(self) -> Option<bool> {
        self.to_bool()
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

    pub fn get_simple_type(&self) -> SimpleModelValueType {
        match self {
            ModelValue::String(_) => SimpleModelValueType::String,
            ModelValue::Number(value) => SimpleModelValueType::from_numeric_value(*value),
            ModelValue::Bool(_) => SimpleModelValueType::Bool,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ModelInput {
    Audio(ControlChannels),
    Sidechain,
    Midi,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, IsVariant, JsonSchema)]
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
#[derive(Clone, Debug, Serialize, Deserialize, Default, JsonSchema)]
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

impl Model {
    pub fn default_reports(&self) -> HashMap<ReportId, MultiChannelTimestampedValue> {
        self.reports
            .iter()
            .map(|(k, v)| (k.clone(), iter::repeat(None).take(v.scope.len(self)).collect()))
            .collect()
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ModelCapability {
    PowerDistributor,
    AudioRouter,
    AudioMixer,
    DigitalInputOutput,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant, Unwrap, JsonSchema)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ChannelParameterRole {
    Pan,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GlobalParameterRole {
    Enable,
    Bypass,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AmplifierParameterRole {
    Enable,
    Gain,
    Distortion,
    SlewRate,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant, JsonSchema)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FilterParameterRole {
    Gain,
    GainDirection,
    Frequency,
    Bandwidth,
    Type,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant, Unwrap, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ModelReportRole {
    #[unwrap(ignore)]
    NoRole,
    Power(PowerReportRole),
    Amplifier(AmplifierId, AmplifierReportRole),
    Dynamics(DynamicsId, DynamicsReportRole),
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PowerReportRole {
    Powered,
    Current,
    PowerFactor,
    TotalEnergy,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AmplifierReportRole {
    PeakVolume,
    RmsVolume,
    LufsVolumeMomentary,
    LufsVolumeShortTerm,
    LufsVolumeIntegrated,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DynamicsReportRole {
    GainReduction,
    GainReductionLimitHit,
}

// #[serde_as]
// #[derive(Serialize, Deserialize, Deref, DerefMut, Debug, Clone, PartialEq, From, Constructor)]
// pub struct MultiChannelValue(#[serde_as(as = "Vec<(_, _)>")] HashMap<usize, ModelValue>);

// alternative
pub type MultiChannelValue = Vec<Option<ModelValue>>;

// #[serde_as]
// #[derive(Serialize, Deserialize, Deref, DerefMut, Debug, Clone, PartialEq, From, Constructor)]
// pub struct MultiChannelTimestampedValue(#[serde_as(as = "Vec<(_, _)>")] HashMap<usize, Timestamped<ModelValue>>);
pub type MultiChannelTimestampedValue = Vec<Option<Timestamped<ModelValue>>>;

pub fn enumerate_multi_channel_value(val: MultiChannelValue) -> impl Iterator<Item = (usize, ModelValue)> {
    val.into_iter().enumerate().filter_map(|(i, v)| v.map(|v| (i, v)))
}

pub fn enumerate_multi_channel_value_bool(val: MultiChannelValue) -> impl Iterator<Item = (usize, bool)> {
    val.into_iter()
       .enumerate()
       .filter_map(|(i, v)| v.and_then(ModelValue::into_bool).map(|v| (i, v)))
}

pub fn enumerate_multi_channel_value_f64(val: MultiChannelValue) -> impl Iterator<Item = (usize, f64)> {
    val.into_iter()
       .enumerate()
       .filter_map(|(i, v)| v.and_then(ModelValue::into_f64).map(|v| (i, v)))
}

pub fn enumerate_multi_channel_value_i64(val: MultiChannelValue) -> impl Iterator<Item = (usize, i64)> {
    val.into_iter()
       .enumerate()
       .filter_map(|(i, v)| v.and_then(ModelValue::into_i64).map(|v| (i, v)))
}

pub mod multi_channel_value {
    use std::iter;

    use crate::common::model::{ModelValue, MultiChannelValue};

    pub fn single(channel: usize, value: ModelValue) -> MultiChannelValue {
        iter::repeat(None).take(channel - 1).chain(Some(Some(value)).into_iter()).collect()
    }

    pub fn bool(channel: usize, value: bool) -> MultiChannelValue {
        single(channel, ModelValue::Bool(value))
    }

    pub fn number(channel: usize, value: f64) -> MultiChannelValue {
        single(channel, ModelValue::Number(value))
    }

    pub fn string(channel: usize, value: String) -> MultiChannelValue {
        single(channel, ModelValue::String(value))
    }

    pub fn join(mut first: MultiChannelValue, other: MultiChannelValue) -> MultiChannelValue {
        for (index, value) in other.into_iter().enumerate() {
            if index >= first.len() {
                first.push(value);
            } else if value.is_some() {
                first[index] = value;
            }
        }

        first
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ModelParameter {
    pub scope:  ModelElementScope,
    #[serde(default)]
    pub unit:   ModelValueUnit,
    pub role:   ModelParameterRole,
    pub values: Vec<ModelValueOption>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ModelElementScope {
    Global,
    AllInputs,
    AllOutputs,
    Count(usize),
}

impl ModelElementScope {
    pub fn len(self, model: &Model) -> usize {
        match self {
            ModelElementScope::Global => 1,
            ModelElementScope::AllInputs => model.inputs.len(),
            ModelElementScope::AllOutputs => model.outputs.len(),
            ModelElementScope::Count(num) => num,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ModelReport {
    pub scope:    ModelElementScope,
    #[serde(default)]
    pub unit:     ModelValueUnit,
    pub role:     ModelReportRole,
    pub values:   Vec<ModelValueOption>,
    #[serde(default)]
    pub public:   bool,
    #[serde(default)]
    pub volatile: bool,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ControlChannels {
    Global,
    Left,
    Right,
    Generic,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Display, JsonSchema)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AmplifierId {
    Input,
    Output,
    Global,
    InsertInput,
    InsertOutput,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DynamicsId {
    Total,
    Compressor,
    Gate,
    Limiter,
    DeEsser,
}

pub fn get_values_type(options: &Vec<ModelValueOption>) -> anyhow::Result<ModelValueType> {
    let simple_options = options.iter()
                                .map(ModelValueOption::get_simple_type)
                                .filter_map(Result::ok)
                                .collect::<HashSet<_>>();

    let maybe_numeric_type = simple_options.iter()
                                           .filter(|x| x.is_number())
                                           .copied()
                                           .reduce(|a, b| a.try_widen(b).unwrap());

    let mut other_types = simple_options.into_iter()
                                        .filter(|x| !x.is_number())
                                        .collect::<HashSet<_>>()
                                        .into_iter();

    let first = other_types.next();
    let second = other_types.next();
    let third = other_types.next();

    Ok(match (maybe_numeric_type, first, second, third) {
        (None, None, None, None) => return Err(anyhow!("value without any times, illegal")),
        (Some(numeric), None, None, None) => ModelValueType::Single(numeric),
        (None, Some(first), None, None) => ModelValueType::Single(first),
        (Some(numeric), Some(first), None, None) => ModelValueType::Either(numeric, first),
        (None, Some(first), Some(second), None) => ModelValueType::Either(first, second),
        _ => ModelValueType::Any, //
    })
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, IsVariant, JsonSchema)]
#[serde(untagged)]
pub enum ToggleOr<T> {
    Toggle(bool),
    Value(T),
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, JsonSchema)]
pub struct Stereo<T> {
    pub left:  T,
    pub right: T,
}