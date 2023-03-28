# CredibleCoin

## About

A zero-knowledge proof system to asses cryptocurrency exchange solvency.

## Code Breakdown

CredibleCoin can be broken into the following core modules:

- [`exchange/`](src/exchange/): everything that an exchange needs to be considered as such by our library. This includes a generic exchange interface which all cryptocurrency exchange types need to implement
- [`utils/`](src/utils/): a collection of utilities used by the library and some of the binaries.
- [`accumulator`](src/accumulator/): CredibleCoin's accumulator algorithms:
  - [`uint_typecast`](src/accumulator/uint_typecast.rs/): A set of helper functions to help the `merkle_rs` crate accept arbitrary integer types (and soon maybe strongs if needed)
  - [`binary accumulator`](src/accumulator/binacc/): A binary accumulator module (TODO)
- [`bin`](src/bin/): CredibleCoin's generated CLI/shell binaries:
  - [`publisher`](src/bin/publisher.rs): The CLI/shell program the publisher (the cryptocurrency exchange itself) uses to answer and recieve queries from the asset keepers (the companies)
- [`cli/`](src/cli/): everything that our CLI's need to function (There are distinct folders for the types and functions each CLI uses)

## Running CredibleCoin

Stable Rust is all that is needed to build `credible-coin`. To build simply run

See `Running our binaries` section below for details

NOTE: Currently there is no binary application built for `credible-coin`, aside from the boilerplate main.rs cargo makes for you.

To run the test suite, use:

```console
$ cargo test
```

Build and open the documentation with:

```console
$ cargo doc --open
```
## Running our binaries
### publisher
```console
$ cargo run --bin publisher [CMD] <ARGS>
```
