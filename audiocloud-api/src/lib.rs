//! A crate of audiocloud API definitions and API calls

use schemars::schema::RootSchema;
use schemars::schema_for;

pub use api::*;
pub use common::*;

pub mod api;
pub mod audio_engine;
pub mod cloud;
pub mod common;
pub mod domain;
pub mod instance_driver;

