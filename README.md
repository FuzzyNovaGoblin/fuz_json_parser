# JSON parser for rust

somewhat fast

currently slow with numbers

## Workspace

This project uses a cargo workspace to hold both the library package itself in one crate and the benchmarks are in a different package

The test runner can be run from the workspace directory because `fuz_json_parser` has no binary directory. The bechmarks can also
be run from the workspace directory `$ cargo bench`
