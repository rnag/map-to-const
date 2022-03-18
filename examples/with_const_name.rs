#![deny(warnings)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;

use map_to_const::*;

use std::collections::HashMap;

fn main() {
    sensible_env_logger::init!();

    // This example also demonstrates sorting the map when generating the `const`.
    //
    // The `map_to_const` uses a `BTreeMap` implementation, so the keys in the input
    // HashMap will automatically be sorted in the result.
    let my_map: HashMap<u64, bool> = HashMap::from([(3, true), (1, true), (2, false)]);

    let const_value = map_to_const(&my_map, "my numeric const");

    trace!("Result:");
    trace!("---");
    trace!("{const_value}");

    // later in code, construct the hashmap from the `const` slice:
    // let my_numeric_map = HashMap::from(MY_NUMERIC_CONST);
}
