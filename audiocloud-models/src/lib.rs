use std::collections::HashMap;
use std::iter;

use lazy_static::lazy_static;
use maplit::hashmap;

use audiocloud_api::model::{ControlChannels, InputChannelRole, Model, ModelInputs, ModelOutputs, OutputChannelRole};
use audiocloud_api::newtypes::ModelId;

use crate::audio_cloud::insert::{audio_cloud_insert_id, audio_cloud_insert_model};
use crate::distopik::dual_1084::{distopik_dual_1084_id, distopik_dual_1084_model};
use crate::distopik::summatra::{distopik_summatra_id, distopik_summatra_model};

pub mod audio_cloud;
pub mod cockos;
pub mod distopik;

pub enum Manufacturers {
  AudioCloud,
  Distopik,
  Elysia,
  Bettermaker,
  Cockos,
  Tierra,
  Gyraf,
}

impl ToString for Manufacturers {
  fn to_string(&self) -> String {
    match self {
      | Manufacturers::Distopik => "distopik",
      | Manufacturers::Elysia => "elysia",
      | Manufacturers::Bettermaker => "bettermaker",
      | Manufacturers::Cockos => "cockos",
      | Manufacturers::Tierra => "tierra",
      | Manufacturers::Gyraf => "gyraf",
      | Manufacturers::AudioCloud => "audio_cloud",
    }.to_owned()
  }
}

lazy_static! {
  pub static ref MODELS: HashMap<ModelId, Model> = hashmap! {
    distopik_dual_1084_id() => distopik_dual_1084_model(),
    distopik_summatra_id() => distopik_summatra_model(),
    audio_cloud_insert_id(1,1) => audio_cloud_insert_model(1,1),
    audio_cloud_insert_id(2,2) => audio_cloud_insert_model(2,2),
    audio_cloud_insert_id(24,2) => audio_cloud_insert_model(24,2),
  };
}

pub fn mono_input() -> ModelInputs {
  vec![(ControlChannels::Global, InputChannelRole::Audio)]
}

pub fn mono_output() -> ModelOutputs {
  vec![(ControlChannels::Global, OutputChannelRole::Audio)]
}

pub fn left_and_right_inputs() -> ModelInputs {
  vec![(ControlChannels::Left, InputChannelRole::Audio),
       (ControlChannels::Right, InputChannelRole::Audio)]
}

pub fn left_and_right_outputs() -> ModelOutputs {
  vec![(ControlChannels::Left, OutputChannelRole::Audio),
       (ControlChannels::Right, OutputChannelRole::Audio)]
}

pub fn standard_inputs(count: usize) -> ModelInputs {
  match count {
    | 1 => mono_input(),
    | 2 => left_and_right_inputs(),
    | n => generic_channels(n, InputChannelRole::Audio),
  }
}

pub fn standard_outputs(count: usize) -> ModelOutputs {
  match count {
    | 1 => mono_output(),
    | 2 => left_and_right_outputs(),
    | n => generic_channels(n, OutputChannelRole::Audio),
  }
}

pub fn generic_channels<R: Copy>(count: usize, role: R) -> Vec<(ControlChannels, R)> {
  iter::repeat((ControlChannels::Generic, role)).take(count).collect()
}

pub mod values {
  use audiocloud_api::model::{ModelValue, ModelValueOption};

  pub fn number(value: f64) -> ModelValueOption {
    ModelValueOption::Single(ModelValue::Number(value))
  }

  pub fn integer(value: i32) -> ModelValueOption {
    number(value as f64)
  }

  pub fn bool_false() -> ModelValueOption {
    ModelValueOption::Single(ModelValue::Bool(false))
  }

  pub fn bool_true() -> ModelValueOption {
    ModelValueOption::Single(ModelValue::Bool(true))
  }

  pub fn numbers(from: f64, to: f64) -> ModelValueOption {
    ModelValueOption::Range(ModelValue::Number(from), ModelValue::Number(to))
  }

  pub fn toggle() -> Vec<ModelValueOption> {
    vec![bool_false(), bool_true()]
  }
}
