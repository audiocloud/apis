use lazy_static::lazy_static;
use maplit::hashmap;

use audiocloud_api::model::ModelValue::Number;
use audiocloud_api::model::ModelValueUnit::{Decibels, Unitless};
use audiocloud_api::model::{AmplifierId, AmplifierParameterRole, Model, ModelElementScope, ModelParameter, ModelParameterRole, ModelValueOption};
use audiocloud_api::newtypes::{ModelId, ParameterId};
use audiocloud_api::model::AmplifierId::Input;
use audiocloud_api::model::AmplifierParameterRole::Gain;
use audiocloud_api::model::ModelParameterRole::{Amplifier, NoRole};
use audiocloud_api::model::ModelValueOption::Single;
use audiocloud_api::model::ModelElementScope::AllInputs;

use crate::Manufacturers::Distopik;
use crate::{standard_inputs, standard_outputs};

pub fn distopik_summatra_id() -> ModelId {
  ModelId::new(Distopik.to_string(), "summatra".to_owned())
}

lazy_static! {
  pub static ref INPUT: ParameterId = ParameterId::new("input".to_owned());
  pub static ref BUS_ASSIGN: ParameterId = ParameterId::new("bus_assign".to_owned());
}

pub const MIN_LEVEL: f64 = -60f64;
pub const MAX_LEVEL: f64 = 10f64;

pub const AMPLIFIER_A: f64 = 0f64;
pub const AMPLIFIER_B: f64 = 1f64;
pub const AMPLIFIER_C: f64 = 2f64;

pub fn distopik_summatra_model() -> Model {
  Model { resources:  Default::default(),
          inputs:     standard_inputs(24),
          outputs:    standard_outputs(2),
          parameters: hashmap! {
            INPUT.clone() => input_level(),
            BUS_ASSIGN.clone() => bus_assign()
          },
          reports:    Default::default(),
          media:      false, }
}

fn input_level() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Decibels,
                   role:   Amplifier(Input, Gain),
                   values: vec![ModelValueOption::num_range(MIN_LEVEL, MAX_LEVEL)], }
}

fn bus_assign() -> ModelParameter {
  ModelParameter { scope:  AllInputs,
                   unit:   Unitless,
                   role:   NoRole,
                   values: vec![Single(Number(AMPLIFIER_A)), Single(Number(AMPLIFIER_B)), Single(Number(AMPLIFIER_C))], }
}
