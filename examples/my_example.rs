#![deny(warnings)]
#![warn(rust_2018_idioms)]

// use log::{debug, info, error};
// use std::time::Instant;

use map_to_const::*;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    // TODO
    println!("Hello world!");

    Ok(())
}
