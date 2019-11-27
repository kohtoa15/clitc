extern crate serde_json;

pub mod clitc_error;
pub mod events;
pub mod params;

use std::{
    time::Instant,
    fs::File,
};
use crate::params::{
    ParamValue,
    CliParameters,
};
use crate::events::{
    WhitespaceSplitter,
    EventHandler,
    Event,
};
