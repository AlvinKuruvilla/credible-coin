# CredibleCoin

## About

A zero-knowledge proof system to asses cryptocurrency exchange solvency.

## Code Breakdown

CredibleCoin can be broken into the following core modules:

- [`credible-coin/`](credible-coin/): Rust crate implementing our solvency protocol and corresponding test suite. See the README.md in the crate for more details

## Running CredibleCoin

Stable Rust is all that is needed to build `credible-coin`. To build simply run

```console
$ cd credible-coin
$ cargo build --release
```

NOTE: Currently there is no binary application built for `credible-coin`, aside from the boilerplate main.rs cargo makes for you.

To run the test suite, use:

```console
$ cd credible-coin
$ cargo test
```

Build and open the documentation with:

```console
$ cd credible-coin
$ cargo doc --open
```
