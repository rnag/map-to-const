#![deny(warnings)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;

use map_to_const::*;

use std::collections::HashMap;

fn main() {
    sensible_env_logger::init!();

    // Create a HashMap in some manner. Ideally, this will be formatted and returned
    // in an API response or similar.
    let my_map = HashMap::from([("hello", "world"), ("testing", "123")]);

    let const_value = map_to_const(&my_map, "my const name");

    trace!("Result:");
    trace!("---");
    trace!("{const_value}");

    // later in code, construct the hashmap from the `const` slice:
    // let my_map = HashMap::from(MY_MAP);
}
