use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::change::{PlayId, RenderId};
use crate::driver::InstanceDriverCommand;
use crate::model::MultiChannelValue;
use crate::newtypes::{ParameterId, ReportId};

#[derive(PartialEq, Serialize, Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum InstancePlayState {
    PreparingToPlay { play_id: PlayId },
    PreparingToRender { length: f64, render_id: RenderId },
    Playing { play_id: PlayId },
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
            DesiredInstancePlayState::Rendering { length, render_id } => {
                InstanceDriverCommand::Render { render_id, length }
            }
            DesiredInstancePlayState::Stopped => InstanceDriverCommand::Stop,
        }
    }
}

impl InstancePlayState {
    pub fn satisfies(&self, required: &DesiredInstancePlayState) -> bool {
        match (self, required) {
            (
                Self::Playing { play_id },
                DesiredInstancePlayState::Playing {
                    play_id: desired_play_id,
                },
            ) => play_id == desired_play_id,
            (
                Self::Rendering { render_id, .. },
                DesiredInstancePlayState::Rendering {
                    render_id: desired_render_id,
                    ..
                },
            ) => render_id == desired_render_id,
            (Self::Stopped, DesiredInstancePlayState::Stopped) => true,
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InstancePowerState {
    WarmingUp,
    CoolingDown,
    PoweredUp,
    ShutDown,
}

impl InstancePowerState {
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
