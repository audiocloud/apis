use lazy_static::lazy_static;
use maplit::hashmap;

use audiocloud_api::model::AmplifierId::{InsertInput, InsertOutput};
use audiocloud_api::model::AmplifierReportRole::PeakVolume;
use audiocloud_api::model::ModelElementScope::AllInputs;
use audiocloud_api::model::ModelReportRole::Amplifier;
use audiocloud_api::model::ModelValueUnit::Decibels;
use audiocloud_api::model::{
    AmplifierId, AmplifierReportRole, Model, ModelElementScope, ModelReport, ModelReportRole, ModelValueOption, ModelValueUnit,
};
use audiocloud_api::newtypes::{ModelId, ReportId};

use crate::Manufacturers::AudioCloud;
use crate::{standard_inputs, standard_outputs};

pub fn audio_cloud_insert_id(input_count: usize, output_count: usize) -> ModelId {
    ModelId::new(AudioCloud.to_string(), format!("insert_{input_count}x{output_count}"))
}

lazy_static! {
    pub static ref INSERT_PREFIX: String = "insert".to_owned();
    pub static ref PEAK_INPUT_LEVEL: ReportId = ReportId::new(format!("{}/input", *INSERT_PREFIX));
    pub static ref PEAK_OUTPUT_LEVEL: ReportId = ReportId::new(format!("{}/output", *INSERT_PREFIX));
}

pub const MAX_REPORTED_LEVEL: f64 = -60f64;

pub fn audio_cloud_insert_model(input_count: usize, output_count: usize) -> Model {
    Model { inputs: standard_inputs(input_count),
            outputs: standard_outputs(output_count),
            reports: hashmap! {
              PEAK_INPUT_LEVEL.clone() => peak_input_level(),
              PEAK_OUTPUT_LEVEL.clone() => peak_output_level()
            },
            ..Default::default() }
}

fn peak_input_level() -> ModelReport {
    ModelReport { scope:    AllInputs,
                  values:   vec![ModelValueOption::to_zero(MAX_REPORTED_LEVEL)],
                  public:   true,
                  role:     Amplifier(InsertInput, PeakVolume),
                  unit:     Decibels,
                  volatile: false, }
}

fn peak_output_level() -> ModelReport {
    ModelReport { scope:    ModelElementScope::AllOutputs,
                  values:   vec![ModelValueOption::to_zero(MAX_REPORTED_LEVEL)],
                  public:   true,
                  role:     Amplifier(InsertOutput, PeakVolume),
                  unit:     Decibels,
                  volatile: false, }
}
