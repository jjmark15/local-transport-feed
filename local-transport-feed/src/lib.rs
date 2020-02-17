#![deny(missing_docs)]

//! The `local-transport-feed` crate provides a simple wrapper around [Transport API](https://transportapi.com).

#[macro_use]
extern crate log;

pub mod client;
pub mod domain;
pub mod services;
