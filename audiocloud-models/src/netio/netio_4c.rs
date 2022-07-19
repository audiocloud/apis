use maplit::hashmap;

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

pub mod params {
    use audiocloud_api::newtypes::ParameterId;

    lazy_static::lazy_static! {
        pub static ref POWER: ParameterId = ParameterId::from("power");
    }
}

pub mod reports {
    use audiocloud_api::newtypes::ReportId;

    lazy_static::lazy_static! {
        pub static ref POWER: ReportId = ReportId::from("power");
        pub static ref CURRENT: ReportId = ReportId::from("current");
        pub static ref POWER_FACTOR: ReportId = ReportId::from("power_factor");
        pub static ref ENERGY: ReportId = ReportId::from("energy");
    }
}

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
            public: true
        },
        reports::CURRENT.clone() => ModelReport {
            scope: Size(NUM_OUTPUTS),
            values: vec![values::numbers(0.0, 10.0)],
            role: Power(Current),
            unit: Amperes,
            public: true
        },
        reports::POWER_FACTOR.clone() => ModelReport {
            scope: Size(NUM_OUTPUTS),
            values: vec![values::numbers(0.0, 1.0)],
            role: Power(PowerFactor),
            unit: Unitless,
            public: false
        },
        reports::ENERGY.clone() => ModelReport {
            scope: Size(NUM_OUTPUTS),
            values: vec![values::numbers(0.0, f64::MAX)],
            role: Power(TotalEnergy),
            unit: WattHours,
            public: false
        },
    };

    Model { resources:  Default::default(),
            inputs:     vec![],
            outputs:    vec![],
            parameters: params,
            reports:    reps,
            media:      false, }
}
