use serde::{Deserialize, Serialize};

use crate::change::{PlayId, RenderId};
use crate::driver::InstanceDriverCommand;
use crate::time::Timestamped;

#[derive(PartialEq, Serialize, Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum InstancePlayState {
    PreparingToPlay { play_id: PlayId },
    Playing { play_id: PlayId },
    PreparingToRender { length: f64, render_id: RenderId },
    Rendering { length: f64, render_id: RenderId },
    Rewinding { to: f64 },
    Stopping,
    Stopped,
}

#[derive(PartialEq, Serialize, Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DesiredInstancePlayState {
    Playing { play_id: PlayId },
    Rendering { length: f64, render_id: RenderId },
    Stopped,
}

impl Into<InstanceDriverCommand> for DesiredInstancePlayState {
    fn into(self) -> InstanceDriverCommand {
        match self {
            DesiredInstancePlayState::Playing { play_id } => InstanceDriverCommand::Play { play_id },
            DesiredInstancePlayState::Rendering { length, render_id } => InstanceDriverCommand::Render { render_id, length },
            DesiredInstancePlayState::Stopped => InstanceDriverCommand::Stop,
        }
    }
}

impl InstancePlayState {
    pub fn satisfies(&self, required: &DesiredInstancePlayState) -> bool {
        match (self, required) {
            (Self::Playing { play_id }, DesiredInstancePlayState::Playing { play_id: desired_play_id }) => play_id == desired_play_id,
            (Self::Rendering { render_id, .. },
             DesiredInstancePlayState::Rendering { render_id: desired_render_id,
                                                   .. }) => render_id == desired_render_id,
            (Self::Stopped, DesiredInstancePlayState::Stopped) => true,
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InstancePowerState {
    PoweringUp,
    ShuttingDown,
    PoweredUp,
    ShutDown,
}

impl InstancePowerState {
    pub fn from_bool(power: bool) -> Self {
        match power {
            true => Self::PoweredUp,
            false => Self::ShutDown,
        }
    }

    pub fn satisfies(self, desired: DesiredInstancePowerState) -> bool {
        match (self, desired) {
            (Self::PoweredUp, DesiredInstancePowerState::PoweredUp) => true,
            (Self::ShutDown, DesiredInstancePowerState::ShutDown) => true,
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DesiredInstancePowerState {
    PoweredUp,
    ShutDown,
}

impl DesiredInstancePowerState {
    pub fn to_bool(self) -> bool {
        match self {
            DesiredInstancePowerState::PoweredUp => true,
            DesiredInstancePowerState::ShutDown => false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ReportInstancePowerState {
    pub desired: Timestamped<DesiredInstancePowerState>,
    pub actual:  Timestamped<InstancePowerState>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ReportInstancePlayState {
    pub desired: Timestamped<DesiredInstancePlayState>,
    pub actual:  Timestamped<InstancePlayState>,
    pub media:   Option<Timestamped<f64>>,
}

pub mod power {
    pub mod params {
        use crate::newtypes::ParameterId;

        lazy_static::lazy_static! {
            pub static ref POWER: ParameterId = ParameterId::from("power");
        }
    }

    pub mod reports {
        use crate::newtypes::ReportId;

        lazy_static::lazy_static! {
            pub static ref POWER: ReportId = ReportId::from("power");
            pub static ref CURRENT: ReportId = ReportId::from("current");
            pub static ref POWER_FACTOR: ReportId = ReportId::from("power_factor");
            pub static ref ENERGY: ReportId = ReportId::from("energy");
        }
    }
}
