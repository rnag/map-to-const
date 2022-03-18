# map-to-const

[<img alt="github" src="https://img.shields.io/badge/github-rnag/map--to--const-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/rnag/map-to-const)
[<img alt="crates.io" src="https://img.shields.io/crates/v/map-to-const.svg?style=for-the-badge&color=fc8d62&logo=rust" height="22">](https://crates.io/crates/map-to-const)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/map-to-const/latest?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="22">](https://docs.rs/map-to-const)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/rnag/map-to-const/build/main?style=for-the-badge" height="22">](https://github.com/rnag/map-to-const/actions?query=branch%3Amain)

Easily convert HashMap<K, V> to constant [(K, V); N] values.

---

This crate works with Cargo with a `Cargo.toml` like:

<!-- Note: the `tokio` dependency can be omitted if this crate doesn't
require any `async` features. -->
```toml
[dependencies]
map-to-const = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

## Getting started

Add some usage to your application.

Here's an example of using `map-to-const` in code:

```rust
use map_to_const::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello world!");

    Ok(())
}
```

## Examples

You can check out sample usage of this crate in the [examples/](https://github.com/rnag/map-to-const/tree/main/examples)
folder in the project repo on GitHub.

## Contributing

Contributions are welcome! Open a pull request to fix a bug, or [open an issue][]
to discuss a new feature or change.

Check out the [Contributing][] section in the docs for more info.

[Contributing]: CONTRIBUTING.md
[open an issue]: https://github.com/rnag/map-to-const/issues

## License

This project is proudly licensed under the MIT license ([LICENSE](LICENSE)
or http://opensource.org/licenses/MIT).

`map-to-const` can be distributed according to the MIT license. Contributions
will be accepted under the same license.

## Authors

* [Ritvik Nag](https://github.com/rnag)
