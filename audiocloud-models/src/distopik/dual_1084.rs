// use std::alloc::Global;

use lazy_static::lazy_static;
use maplit::hashmap;

use audiocloud_api::model::AmplifierId::{Input, Output};
use audiocloud_api::model::AmplifierParameterRole::Gain as AmpGain;
use audiocloud_api::model::FilterParameterRole::{Bandwidth, Frequency, Gain as FilterGain};
use audiocloud_api::model::GlobalParameterRole::Bypass;
use audiocloud_api::model::ModelElementScope::AllInputs;
use audiocloud_api::model::ModelElementScope::AllOutputs;
use audiocloud_api::model::ModelParameterRole::{Amplifier, Filter, Global};
use audiocloud_api::model::ModelValueUnit::{Decibels, Hertz, Toggle};
use audiocloud_api::model::{Model, ModelElementScope, ModelParameter, ModelValueOption};
use audiocloud_api::newtypes::FilterId::{High, HighMid, HighPass, Low, LowMid};
use audiocloud_api::newtypes::{ModelId, ParameterId};

use crate::Manufacturers::Distopik;
use crate::{left_and_right_inputs, left_and_right_outputs, values};

pub fn distopik_dual_1084_id() -> ModelId {
    ModelId::new(Distopik.to_string(), "dual_1084".to_owned())
}

lazy_static! {
  // --- input section
  pub static ref INPUT_GAIN: ParameterId = ParameterId::from("input_gain");
  pub static ref HIGH_PASS_FILTER: ParameterId = ParameterId::from("high_pass_filter");
  // --- low eq section
  pub static ref LOW_FREQ: ParameterId = ParameterId::from("low_freq");
  pub static ref LOW_GAIN: ParameterId = ParameterId::from("low_gain");
  // --- low mid eq section
  pub static ref LOW_MID_FREQ: ParameterId = ParameterId::from("low_mid_freq");
  pub static ref LOW_MID_GAIN: ParameterId = ParameterId::from("low_mid_gain");
  pub static ref LOW_MID_WIDTH: ParameterId = ParameterId::from("low_mid_width");
  // --- high mid eq section
  pub static ref HIGH_MID_FREQ: ParameterId = ParameterId::from("high_mid_freq");
  pub static ref HIGH_MID_GAIN: ParameterId = ParameterId::from("high_mid_gain");
  pub static ref HIGH_MID_WIDTH: ParameterId = ParameterId::from("high_mid_width");
  // --- high eq section
  pub static ref HIGH_FREQ: ParameterId = ParameterId::from("high_freq");
  pub static ref HIGH_GAIN: ParameterId = ParameterId::from("high_gain");
  // --- output section
  pub static ref OUTPUT_PAD: ParameterId = ParameterId::from("output_pad");
  pub static ref EQL_TOGGLE: ParameterId = ParameterId::from("eql_toggle");
}

pub fn distopik_dual_1084_model() -> Model {
    let params = hashmap! {
      INPUT_GAIN.clone() => input_gain(),
      HIGH_PASS_FILTER.clone() => high_pass_filter(),
      LOW_FREQ.clone() => low_freq(),
      LOW_GAIN.clone() => low_gain(),
      LOW_MID_FREQ.clone() => low_mid_freq(),
      LOW_MID_GAIN.clone() => low_mid_gain(),
      LOW_MID_WIDTH.clone() => low_mid_width(),
      HIGH_MID_FREQ.clone() => high_mid_freq(),
      HIGH_MID_GAIN.clone() => high_mid_gain(),
      HIGH_MID_WIDTH.clone() => high_mid_width(),
      HIGH_FREQ.clone() => high_freq(),
      HIGH_GAIN.clone() => high_gain(),
      OUTPUT_PAD.clone() => output_pad(),
      EQL_TOGGLE.clone() => eql_toggle()
    };

    Model { inputs:       left_and_right_inputs(),
            outputs:      left_and_right_outputs(),
            parameters:   params,
            resources:    Default::default(),
            reports:      Default::default(),
            media:        false,
            capabilities: Default::default(), }
}

pub fn input_gain() -> ModelParameter {
    ModelParameter { scope:  AllInputs,
                     unit:   Decibels,
                     role:   Amplifier(Input, AmpGain),
                     values: vec![values::bool_false(),
                                  values::integer(-10),
                                  values::integer(-5),
                                  values::integer(0),
                                  values::integer(5),
                                  values::integer(10),
                                  values::integer(15),
                                  values::integer(20),], }
}
pub fn high_pass_filter() -> ModelParameter {
    ModelParameter { scope:  AllInputs,
                     unit:   Hertz,
                     role:   Filter(HighPass, Frequency),
                     values: vec![values::bool_false(),
                                  values::integer(22),
                                  values::integer(45),
                                  values::integer(70),
                                  values::integer(160),
                                  values::integer(360),], }
}
pub fn low_freq() -> ModelParameter {
    ModelParameter { scope:  AllInputs,
                     unit:   Hertz,
                     role:   Filter(Low, Frequency),
                     values: vec![values::bool_false(),
                                  values::integer(20),
                                  values::integer(35),
                                  values::integer(60),
                                  values::integer(110),
                                  values::integer(220),], }
}
pub fn low_gain() -> ModelParameter {
    ModelParameter { scope:  AllInputs,
                     unit:   Decibels,
                     role:   Filter(Low, FilterGain),
                     values: filter_gain_values_16(), }
}
pub fn low_mid_freq() -> ModelParameter {
    ModelParameter { scope:  AllInputs,
                     unit:   Hertz,
                     role:   Filter(LowMid, Frequency),
                     values: vec![values::bool_false(),
                                  values::integer(120),
                                  values::integer(180),
                                  values::integer(240),
                                  values::integer(360),
                                  values::integer(480),
                                  values::integer(720),
                                  values::integer(1_600),
                                  values::integer(2_400),
                                  values::integer(3_200),
                                  values::integer(4_800),
                                  values::integer(7_200),], }
}
pub fn low_mid_gain() -> ModelParameter {
    ModelParameter { scope:  AllInputs,
                     unit:   Decibels,
                     role:   Filter(LowMid, FilterGain),
                     values: filter_gain_values_12(), }
}
pub fn low_mid_width() -> ModelParameter {
    ModelParameter { scope:  AllInputs,
                     unit:   Toggle,
                     role:   Filter(LowMid, Bandwidth),
                     values: values::toggle(), }
}
pub fn high_mid_freq() -> ModelParameter {
    ModelParameter { scope:  AllInputs,
                     unit:   Hertz,
                     role:   Filter(HighMid, Frequency),
                     values: vec![values::bool_false(),
                                  values::integer(360),
                                  values::integer(480),
                                  values::integer(720),
                                  values::integer(1_600),
                                  values::integer(2_400),
                                  values::integer(3_200),
                                  values::integer(3_900),
                                  values::integer(4_800),
                                  values::integer(6_400),
                                  values::integer(7_200),
                                  values::integer(8_400),], }
}
pub fn high_mid_gain() -> ModelParameter {
    ModelParameter { scope:  AllInputs,
                     unit:   Decibels,
                     role:   Filter(HighMid, FilterGain),
                     values: filter_gain_values_12(), }
}
pub fn high_mid_width() -> ModelParameter {
    ModelParameter { scope:  AllInputs,
                     unit:   Toggle,
                     role:   Filter(HighMid, Bandwidth),
                     values: values::toggle(), }
}
pub fn high_freq() -> ModelParameter {
    ModelParameter { scope:  AllInputs,
                     unit:   Hertz,
                     role:   Filter(High, Frequency),
                     values: vec![values::bool_false(),
                                  values::integer(8_000),
                                  values::integer(10_000),
                                  values::integer(12_000),
                                  values::integer(16_000),
                                  values::integer(20_000),], }
}
pub fn high_gain() -> ModelParameter {
    ModelParameter { scope:  AllInputs,
                     unit:   Decibels,
                     role:   Filter(High, FilterGain),
                     values: filter_gain_values_16(), }
}

pub fn output_pad() -> ModelParameter {
    ModelParameter { scope:  AllOutputs,
                     unit:   Decibels,
                     role:   Amplifier(Output, AmpGain),
                     values: vec![values::bool_false(), values::integer(-10), values::integer(-20)], }
}

pub fn eql_toggle() -> ModelParameter {
    ModelParameter { scope:  AllInputs,
                     unit:   Toggle,
                     role:   Global(Bypass),
                     values: values::toggle(), }
}

fn filter_gain_values_16() -> Vec<ModelValueOption> {
    vec![values::numbers(-16_f64, 16_f64)]
}

fn filter_gain_values_12() -> Vec<ModelValueOption> {
    vec![values::numbers(-12_f64, 12_f64)]
}
