# Examples

This folder contains example scripts that can be used to interact with
the `map-to-const` crate.

## Quickstart

First, start out by cloning the GitHub project:

```shell
❯❯ git clone https://github.com/rnag/map-to-const.git
```

When running the examples, we'll often want to see the DEBUG logs from the library under test,
`map-to-const` in this case. Therefore, remember to ensure that the **RUST_LOG** env variable
is properly set.

For example, on *Mac/Linux*:

```shell
❯❯ export RUST_LOG='map_to_const=TRACE'
```

On *Windows*:

```shell
❯❯ $env:RUST_LOG='map_to_const=TRACE'
```

Next, simply just `cd` into the project folder:

```shell
❯❯ cd map-to-const
```

From here, you can use `cargo` to build and run
any of the examples individually.

In particular, here's a sample usage of running `examples/my_example.rs`:

```shell
❯❯ cargo run --example my_example
```