use std::collections::HashMap;
use std::iter;

use lazy_static::lazy_static;
use maplit::hashmap;

use audiocloud_api::common::model::{ControlChannels, Model, ModelInput, ModelInputs, ModelOutput, ModelOutputs};
use audiocloud_api::newtypes::ModelId;

use crate::audio_cloud::insert::{audio_cloud_insert_id, audio_cloud_insert_model};
use crate::distopik::dual_1084::{distopik_dual_1084_id, distopik_dual_1084_model};
use crate::distopik::summatra::{distopik_summatra_id, distopik_summatra_model};
use crate::netio::netio_4c::{netio_power_pdu_4c_id, netio_power_pdu_4c_model};

pub mod audio_cloud;
pub mod cockos;
pub mod distopik;
pub mod netio;

pub enum Manufacturers {
    AudioCloud,
    Distopik,
    Elysia,
    Bettermaker,
    Cockos,
    Tierra,
    Gyraf,
    Netio,
}

impl ToString for Manufacturers {
    fn to_string(&self) -> String {
        match self {
            Manufacturers::Distopik => "distopik",
            Manufacturers::Elysia => "elysia",
            Manufacturers::Bettermaker => "bettermaker",
            Manufacturers::Cockos => "cockos",
            Manufacturers::Tierra => "tierra",
            Manufacturers::Gyraf => "gyraf",
            Manufacturers::AudioCloud => "audio_cloud",
            Manufacturers::Netio => "netio",
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
      netio_power_pdu_4c_id() => netio_power_pdu_4c_model()
    };
}

pub fn mono_input() -> ModelInputs {
    vec![(ModelInput::Audio(ControlChannels::Global))]
}

pub fn mono_output() -> ModelOutputs {
    vec![(ModelOutput::Audio(ControlChannels::Global))]
}

pub fn left_and_right_inputs() -> ModelInputs {
    vec![ModelInput::Audio(ControlChannels::Left), ModelInput::Audio(ControlChannels::Right),]
}

pub fn left_and_right_outputs() -> ModelOutputs {
    vec![ModelOutput::Audio(ControlChannels::Left),
         ModelOutput::Audio(ControlChannels::Right),]
}

pub fn standard_inputs(count: usize) -> ModelInputs {
    match count {
        1 => mono_input(),
        2 => left_and_right_inputs(),
        n => repeat_channels(n, ModelInput::Audio(ControlChannels::Generic)),
    }
}

pub fn standard_outputs(count: usize) -> ModelOutputs {
    match count {
        1 => mono_output(),
        2 => left_and_right_outputs(),
        n => repeat_channels(n, ModelOutput::Audio(ControlChannels::Generic)),
    }
}

pub fn repeat_channels<R: Copy>(count: usize, role: R) -> Vec<R> {
    iter::repeat(role).take(count).collect()
}

pub mod values {
    use audiocloud_api::common::model::{ModelValue, ModelValueOption};

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
