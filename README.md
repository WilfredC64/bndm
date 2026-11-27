# BNDM

[![Rust](https://github.com/WilfredC64/bndm/actions/workflows/rust.yml/badge.svg)](https://github.com/WilfredC64/bndm/actions/workflows/rust.yml)
[![Latest version](https://img.shields.io/crates/v/bndm.svg)](https://crates.io/crates/bndm)
[![Documentation](https://docs.rs/bndm/badge.svg)](https://docs.rs/bndm)
![License](https://img.shields.io/crates/l/bndm.svg)

A Rust library that implements the BNDM algorithm for fast and efficient pattern matching, with support for wildcard searches.

## Overview

BNDM (Backward Nondeterministic Dawg Matching) is an optimized string search algorithm designed for efficiently locating patterns within a text. The BNDM algorithm was invented by Gonzalo Navarro and Mathieu Raffinot.

The BNDM algorithm works by preprocessing the pattern to generate a set of bitmasks. These bitmasks are then used to efficiently scan the text for occurrences of the pattern.

Unlike the traditional BNDM algorithm, this implementation is optimized by checking the last 2 bytes of each window before entering the inner loop. Additionally, it does not have the limitation that the pattern size is limited to the word size of the CPU. It's possible to search for larger patterns than the word size and still benefit from the performance that BNDM offers.

One of the key features of this implementation is its support for wildcard search, for which BNDM is ideally suited. Wildcards are encoded in the bitmasks during preprocessing, so the BNDM scan does exactly the same work per byte with or without wildcards. Where many other search algorithms need extra comparisons or special handling to support wildcards, the BNDM scan supports them at no additional cost per byte.

## Usage

Here is an example of how to use the library to search for a pattern in a text:

### Without wildcard

```rust
use bndm::{BndmConfig, find_pattern};

let source = b"The quick brown fox jumps over the lazy dog";
let pattern = b"jumps";
let config = BndmConfig::new(pattern, None);
let index = find_pattern(source, &config);
assert_eq!(index, Some(20));
```

### With wildcard

```rust
use bndm::{BndmConfig, find_pattern};

let source = b"The quick brown fox jumps over the lazy dog";
let pattern = b"ju??s";
let config = BndmConfig::new(pattern, Some(b'?'));
let index = find_pattern(source, &config);
assert_eq!(index, Some(20));
```

## Copyright

Copyright &#xa9; 2019 - 2026 by Wilfred Bos.

## License

This project is licensed under the [MIT License](/LICENSE).
