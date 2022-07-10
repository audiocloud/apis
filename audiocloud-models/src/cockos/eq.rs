use std::collections::HashMap;

use lazy_static::lazy_static;
use maplit::hashmap;

use audiocloud_api::model::FilterParameterRole::{Frequency, Gain};
use audiocloud_api::model::ModelElementScope::AllInputs;
use audiocloud_api::model::ModelParameterRole::Filter;
use audiocloud_api::model::ModelValueUnit::Decibels;
use audiocloud_api::model::ResourceId::CPU;
use audiocloud_api::model::{Model, ModelParameter, ModelValueOption, ResourceId};
use audiocloud_api::newtypes::FilterId::Low;
use audiocloud_api::newtypes::{ModelId, ParameterId};

use crate::Manufacturers::Cockos;
use crate::{standard_inputs, standard_outputs, values};

lazy_static! {
  pub static ref LOW_GAIN: ParameterId = ParameterId::from("low_gain");
  pub static ref LOW_FREQ: ParameterId = ParameterId::from("low_freq");
}

pub fn cockos_eq_id() -> ModelId {
  ModelId::new(Cockos.to_string(), "eq".to_string())
}

pub fn cockos_eq() -> Model {
  let resources = hashmap! {
    CPU => 100_f64
  };

  let inputs = standard_inputs(2);
  let outputs = standard_outputs(2);

  let parameters = hashmap! {
    LOW_GAIN.clone() => low_gain(),
    LOW_FREQ.clone() => low_freq()
  };

  let reports = HashMap::new();

  let media = false;

  Model { resources,
          inputs,
          outputs,
          parameters,
          reports,
          media }
}

fn low_gain() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Decibels,
                   role:   Filter(Low, Gain),
                   values: filter_gain_numbers(), }
}

fn low_freq() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Decibels,
                   role:   Filter(Low, Frequency),
                   values: vec![values::numbers(20_f64, 2000_f64)], }
}

fn filter_gain_numbers() -> Vec<ModelValueOption> {
  vec![values::numbers(-48_f64, 48_f64)]
}
