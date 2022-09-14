use audiocloud_api::model::*;
use audiocloud_api::api::*;
use serde::{Serialize, Deserialize};
use schemars::{JsonSchema, schema_for};
use schemars::schema::RootSchema;


pub mod distopik {

use super::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Dual1084Preset {
    pub low_mid_freq: Stereo<ToggleOr<u64>>,
    pub low_mid_gain: Stereo<f64>,
    pub low_freq: Stereo<ToggleOr<u64>>,
    pub high_gain: Stereo<f64>,
    pub high_pass_filter: Stereo<ToggleOr<u64>>,
    pub high_mid_width: Stereo<bool>,
    pub low_gain: Stereo<f64>,
    pub high_mid_freq: Stereo<ToggleOr<u64>>,
    pub high_mid_gain: Stereo<f64>,
    pub high_freq: Stereo<ToggleOr<u64>>,
    pub low_mid_width: Stereo<bool>,
    pub eql_toggle: Stereo<bool>,
    pub input_gain: Stereo<ToggleOr<i64>>,
    pub output_pad: Stereo<ToggleOr<i64>>,}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Dual1084Parameters {
    pub low_mid_freq: Option<Stereo<ToggleOr<u64>>>,
    pub low_mid_gain: Option<Stereo<f64>>,
    pub low_freq: Option<Stereo<ToggleOr<u64>>>,
    pub high_gain: Option<Stereo<f64>>,
    pub high_pass_filter: Option<Stereo<ToggleOr<u64>>>,
    pub high_mid_width: Option<Stereo<bool>>,
    pub low_gain: Option<Stereo<f64>>,
    pub high_mid_freq: Option<Stereo<ToggleOr<u64>>>,
    pub high_mid_gain: Option<Stereo<f64>>,
    pub high_freq: Option<Stereo<ToggleOr<u64>>>,
    pub low_mid_width: Option<Stereo<bool>>,
    pub eql_toggle: Option<Stereo<bool>>,
    pub input_gain: Option<Stereo<ToggleOr<i64>>>,
    pub output_pad: Option<Stereo<ToggleOr<i64>>>,}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Dual1084Reports {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SummatraPreset {
    pub input: Vec<f64>,
    pub pan: Vec<f64>,
    pub bus_assign: Vec<u64>,}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SummatraParameters {
    pub input: Option<Vec<f64>>,
    pub pan: Option<Vec<f64>>,
    pub bus_assign: Option<Vec<u64>>,}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SummatraReports {}

}

pub mod netio {

use super::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PowerPdu4CPreset {
    pub power: Vec<bool>,}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PowerPdu4CParameters {
    pub power: Option<Vec<bool>>,}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PowerPdu4CReports {
    pub power_factor: Option<Vec<f64>>,
    pub power: Option<Vec<bool>>,
    pub energy: Option<Vec<f64>>,
    pub current: Option<Vec<f64>>,}

}

pub mod audio {

use super::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CloudInsert1X1Preset {}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CloudInsert1X1Parameters {}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CloudInsert1X1Reports {
    pub insert_output: Option<f64>,
    pub insert_input: Option<f64>,}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CloudInsert2X2Preset {}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CloudInsert2X2Parameters {}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CloudInsert2X2Reports {
    pub insert_input: Option<Stereo<f64>>,
    pub insert_output: Option<Stereo<f64>>,}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CloudInsert24X2Preset {}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CloudInsert24X2Parameters {}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CloudInsert24X2Reports {
    pub insert_input: Option<Vec<f64>>,
    pub insert_output: Option<Stereo<f64>>,}

}


pub fn schemas() -> RootSchema {
    merge_schemas([
      schema_for!(self::distopik::Dual1084Preset),
      schema_for!(self::distopik::Dual1084Parameters),
      schema_for!(self::distopik::Dual1084Reports),
      schema_for!(self::distopik::SummatraPreset),
      schema_for!(self::distopik::SummatraParameters),
      schema_for!(self::distopik::SummatraReports),
      schema_for!(self::netio::PowerPdu4CPreset),
      schema_for!(self::netio::PowerPdu4CParameters),
      schema_for!(self::netio::PowerPdu4CReports),
      schema_for!(self::audio::CloudInsert1X1Preset),
      schema_for!(self::audio::CloudInsert1X1Parameters),
      schema_for!(self::audio::CloudInsert1X1Reports),
      schema_for!(self::audio::CloudInsert2X2Preset),
      schema_for!(self::audio::CloudInsert2X2Parameters),
      schema_for!(self::audio::CloudInsert2X2Reports),
      schema_for!(self::audio::CloudInsert24X2Preset),
      schema_for!(self::audio::CloudInsert24X2Parameters),
      schema_for!(self::audio::CloudInsert24X2Reports),
    ].into_iter())
}