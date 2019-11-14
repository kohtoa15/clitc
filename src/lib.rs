extern crate serde_json;

mod clitc_error;
mod events;
mod params;

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
