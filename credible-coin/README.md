# CredibleCoin

## About

A zero-knowledge proof system to asses cryptocurrency exchange solvency.

## Code Breakdown

CredibleCoin can be broken into the following core modules:

- [`exchange/`](src/exchange/): everything that an exchange needs to be considered as such by our library. This includes a generic exchange interface which all cryptocurrency exchange types need to implement
- [`utils/`](src/utils/): a collection of utilities used by the library
  to get up and running with.
- [`accumulator`](src/accumulator/): CredibleCoin's accumulator algorithms:
  - [`merkle`](src/accumulator/merkle/): A modified merkle-tree data structure with a MerkleNode wrapper type
  - [`binary accumulator`](src/accumulator/binacc/): A binary accumulator module

## Running CredibleCoin

Stable Rust is all that is needed to build `credible-coin`. To build simply run

```console
$ cargo build --release
```

NOTE: Currently there is no binary application built for `credible-coin`, aside from the boilerplate main.rs cargo makes for you.
To run the test suite, use:

```console
$ cargo test
```

Build and open the documentation with:

```console
$ cargo doc --open
```
