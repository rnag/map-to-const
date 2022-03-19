#![doc(html_root_url = "https://docs.rs/map-to-const/0.2")]
#![warn(rust_2018_idioms, missing_docs)]
#![deny(warnings, dead_code, unused_imports, unused_mut)]

//! [![github]](https://github.com/rnag/map-to-const)&ensp;[![crates-io]](https://crates.io/crates/map-to-const)&ensp;[![docs-rs]](https://docs.rs/map-to-const)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! Easily convert `HashMap<K, V>` to constant `[(K, V); N]` values.
//!
//! <br>
//!
//! ## Usage
//!
//! ```rust
//! use map_to_const::*;
//! use std::collections::HashMap;
//!
//! fn main() {
//!     // Create a HashMap in some manner. Ideally, this will be formatted
//!     // and returned in an API response or similar.
//!     let my_map = HashMap::from([("testing", "123"), ("hello", "world")]);
//!
//!     let const_value = map_to_const(&my_map, None);
//!
//!     println!("{const_value}");
//!
//!     // later in code, construct the hashmap from the `const` slice:
//!     // let my_map = HashMap::from(MY_MAP);
//! }
//! ```
//!
//! ## Examples
//!
//! You can check out sample usage of this crate in the [examples/](https://github.com/rnag/map-to-const/tree/main/examples)
//! folder in the project repo on GitHub.
//!
//! ## Readme Docs
//!
//! You can find the crate's readme documentation on the
//! [crates.io] page, or alternatively in the [`README.md`] file on the GitHub project repo.
//!
//! [crates.io]: https://crates.io/crates/map-to-const
//! [`README.md`]: https://github.com/rnag/map-to-const
//!
use std::collections::{BTreeMap, HashMap};

pub(crate) const DEFAULT_CONST_NAME: &str = "my_map";
pub(crate) const INDENT: &str = "  ";

/// Trait to retrieve the string value (i.e. name) of a type.
///
/// Credits:
/// <https://stackoverflow.com/a/56100816/10237506>
pub trait ExtTypeName {
    /// Retrieve the string value of a type.
    fn type_name(&self) -> &str
    where
        Self: Sized;
}

impl ExtTypeName for String {
    fn type_name(&self) -> &str {
        "&str"
    }
}

macro_rules! impl_type_name {
    (for $($t:ty),+) => {
        $(impl ExtTypeName for $t {

            fn type_name(&self) -> &str {
                stringify!($t)
            }
        })*
    }
}

impl_type_name!(for &str, str, bool, char, usize,
                    u8, u16, u32, u64, u128,
                    i8, i16, i32, i64, i128,
                    f32, f64);

/// Converts a `HashMap<K, V>` to a `const` or constant `[(K, V); N]`
/// string representation, where `N` is the number of key-value pairs in the
/// input *map* object.
///
/// # Example
///
/// ```rust
/// use map_to_const::*;
///
/// let const_value = map_to_const(
///   &std::collections::HashMap::from([("hello", "world"), ("testing", "123")]),
///   "my_const_name"
/// );
/// ```
///
/// # Errors
///
/// Panics if the input `HashMap` object is empty.
///
pub fn map_to_const<
    'a,
    K: ExtTypeName + std::fmt::Debug + std::cmp::Ord,
    V: ExtTypeName + std::fmt::Debug,
>(
    map: &HashMap<K, V>,
    const_name: impl Into<Option<&'a str>>,
) -> String {
    let const_name = const_name
        .into()
        .unwrap_or(DEFAULT_CONST_NAME)
        .replace(' ', "_")
        .replace('-', "_")
        .to_uppercase();

    let map_iter = map.iter();
    let (k, v) = map.iter().nth(0).unwrap();

    let mut const_define = format!(
        "const {name}: [({kt}, {vt}); {len}] = ",
        name = const_name,
        kt = k.type_name(),
        vt = v.type_name(),
        len = map.len()
    );

    const_define.push('[');

    let sorted_map = BTreeMap::from_iter(map_iter);

    for (key, value) in sorted_map {
        let fmt = format!("\n{0}({1:?}, {2:?}),", INDENT, key, value);
        const_define.push_str(&fmt);
    }

    const_define.push('\n');
    const_define.push(']');
    const_define.push(';');

    const_define
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;
    use log::*;
    use std::collections::HashMap;

    #[test]
    fn test_simple() {
        sensible_env_logger::safe_init!();

        let my_map = HashMap::from([("testing", "123"), ("hello", "world")]);

        let const_value = map_to_const(&my_map, None);

        trace!("RESULT:\n---\n{}", const_value);

        assert_eq!(
            const_value,
            indoc! {r#"
                const MY_MAP: [(&str, &str); 2] = [
                  ("hello", "world"),
                  ("testing", "123"),
                ];
            "#}
            .trim_end()
        );
    }

    #[test]
    fn test_with_numeric_keys() {
        sensible_env_logger::safe_init!();

        let my_map = HashMap::from([
            (9876543210u64, "123".to_owned()),
            (1122334455u64, "world".to_owned()),
        ]);

        let const_value = map_to_const(&my_map, "my map value");

        trace!("RESULT:\n---\n{}", const_value);

        assert_eq!(
            const_value,
            indoc! {r#"
                const MY_MAP_VALUE: [(u64, &str); 2] = [
                  (1122334455, "world"),
                  (9876543210, "123"),
                ];
            "#}
            .trim_end()
        );
    }

    #[test]
    fn test_with_boolean_keys() {
        sensible_env_logger::safe_init!();

        let my_map = HashMap::from([(true, 123.45), (false, 54.321)]);

        let const_value = map_to_const(&my_map, "my-bool-map");

        trace!("RESULT:\n---\n{}", const_value);

        assert_eq!(
            const_value,
            indoc! {r#"
                const MY_BOOL_MAP: [(bool, f64); 2] = [
                  (false, 54.321),
                  (true, 123.45),
                ];
            "#}
            .trim_end()
        );
    }
}
