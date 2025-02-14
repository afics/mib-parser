extern crate pest;
extern crate serde;
#[macro_use]
extern crate log;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

mod parser;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MibInfo {
    pub modules: Vec<Module>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub imports: Vec<Import>,
    pub assignments: Vec<Assignment>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Assignment {
    pub name: String,
    pub a_type: String,
    pub value: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Import {
    pub name: String,
    pub from: String,
}

pub struct ParseOptions {
    pub pretty_print: bool,
}

/// Parse a single file
pub fn parse_file<P: AsRef<Path>>(
    mib_file: &P,
    options: &ParseOptions,
) -> Result<MibInfo, Box<dyn std::error::Error>> {
    trace!("Reading {}", mib_file.as_ref().display());
    let mib_string = fs::read_to_string(mib_file)?;
    trace!("Read {} characters", mib_string.len());
    Ok(parser::parse_mib(&mib_string, options)?)
}
