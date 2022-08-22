use maplit::{hashmap, hashset};

pub use audiocloud_api::instance::power::{params, reports};
use audiocloud_api::model::ModelCapability::PowerDistributor;
use audiocloud_api::model::ModelElementScope::Size;
use audiocloud_api::model::ModelParameterRole::Power as PowerOnOff;
use audiocloud_api::model::ModelReportRole::Power;
use audiocloud_api::model::ModelValueUnit::*;
use audiocloud_api::model::PowerReportRole::*;
use audiocloud_api::model::{Model, ModelParameter, ModelReport};
use audiocloud_api::newtypes::ModelId;

use crate::values;
use crate::Manufacturers::Netio;

pub fn netio_power_pdu_4c_id() -> ModelId {
    ModelId::new(Netio.to_string(), "power_pdu_4c".to_owned())
}

const NUM_OUTPUTS: usize = 4;

pub fn netio_power_pdu_4c_model() -> Model {
    let params = hashmap! {
        params::POWER.clone() => ModelParameter {
            scope: Size(NUM_OUTPUTS),
            values: values::toggle(),
            role: PowerOnOff,
            unit: Toggle
        }
    };

    let reps = hashmap! {
        reports::POWER.clone() => ModelReport {
            scope: Size(NUM_OUTPUTS),
            values: values::toggle(),
            role: Power(Powered),
            unit: Toggle,
            public: true,
            volatile: true
        },
        reports::CURRENT.clone() => ModelReport {
            scope: Size(NUM_OUTPUTS),
            values: vec![values::numbers(0.0, 10.0)],
            role: Power(Current),
            unit: Amperes,
            public: true,
            volatile: false
        },
        reports::POWER_FACTOR.clone() => ModelReport {
            scope: Size(NUM_OUTPUTS),
            values: vec![values::numbers(0.0, 1.0)],
            role: Power(PowerFactor),
            unit: Unitless,
            public: false,
            volatile: false
        },
        reports::ENERGY.clone() => ModelReport {
            scope: Size(NUM_OUTPUTS),
            values: vec![values::numbers(0.0, f64::MAX)],
            role: Power(TotalEnergy),
            unit: WattHours,
            public: false,
            volatile: false
        },
    };

    Model { resources:    Default::default(),
            inputs:       vec![],
            outputs:      vec![],
            parameters:   params,
            reports:      reps,
            media:        false,
            capabilities: hashset! {PowerDistributor}, }
}
