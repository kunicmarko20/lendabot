#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate cfg_if;

pub mod command;
pub mod github;

#[cfg(feature = "lambda-slack")]
pub mod slack;

pub use crate::command::Command;
