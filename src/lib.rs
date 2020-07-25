#![allow(dead_code, unused_variables, unreachable_code)]

pub mod uni_dimentional;
pub mod bi_dimentional;
pub mod timeseries;
pub mod generic_types;
mod test;

pub use OrdFloat;
pub use crate::{
    uni_dimentional::*,
    bi_dimentional::*,
    timeseries::*,
    generic_types::*
};