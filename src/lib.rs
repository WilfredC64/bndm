// Copyright (C) 2019 - 2024 Wilfred Bos
// Licensed under the MIT license. See the LICENSE file for the terms and conditions.

//! # Backward Nondeterministic Dawg Matching (BNDM)
//!
//! BNDM is an optimized string search algorithm designed for efficiently locating
//! patterns within a text. The BNDM algorithm was invented by Gonzalo Navarro and Mathieu
//! Raffinot.
//!
//! The BNDM algorithm works by preprocessing the pattern to generate a set of bitmasks.
//! These bitmasks are then used to efficiently scan the text for occurrences of the
//! pattern.
//!
//! Unlike the traditional BNDM algorithm, this implementation is optimized by scanning the
//! first 2 bytes outside the inner loop. Additionally, it does not have the limitation that
//! the pattern size is limited to the word size of the CPU. It's possible to search for
//! larger patterns than the word size and still benefit from the performance that BNDM offers.
//!
//! One of the key features of this implementation is its support for wildcard search.
//! This algorithm is ideally suited for this, as it does not impact the performance of the
//! pattern matching itself. The wildcard search only slightly affects the indexing, ensuring
//! that the overall efficiency of the pattern matching remains high.
//!
//! ## BndmConfig
//!
//! The `BndmConfig` struct is used to store the preprocessed pattern and the bitmasks.
//!
//! ## Functions
//!
//! The main function provided by this module is `find_pattern()`, which searches for the
//! pattern in a given text. It returns the index of the first occurrence of the pattern
//! in the text, or `None` if the pattern is not found.
//!
//! ## Usage
//!
//! Here is an example of how to use this module to search for a pattern in a text:
//!
//! ```rust
//! use bndm::{BndmConfig, find_pattern};
//!
//! let source = b"The quick brown fox jumps over the lazy dog";
//! let pattern = b"jumps";
//! let config = BndmConfig::new(pattern, None);
//! let index = find_pattern(source, &config);
//! assert_eq!(index, Some(20));
//! ```
//!
//! With wildcard:
//!
//! ```rust
//! use bndm::{BndmConfig, find_pattern};
//!
//! let source = b"The quick brown fox jumps over the lazy dog";
//! let pattern = b"ju??s";
//! let config = BndmConfig::new(pattern, Some(b'?'));
//! let index = find_pattern(source, &config);
//! assert_eq!(index, Some(20));
//! ```

use std::cmp::min;

const MASKS_TABLE_SIZE: usize = 256;
const WORD_SIZE_IN_BITS: usize = usize::BITS as usize;

/// The `BndmConfig` struct is used to store the pattern and the bitmasks.
pub struct BndmConfig {
    /// An array of bitmasks, one for each possible byte value.
    pub masks: [usize; MASKS_TABLE_SIZE],

    /// An optional wildcard character. If provided, this character in the pattern
    /// can match any character in the text.
    pub wildcard: Option<u8>,

    /// The pattern to search for in the text.
    pub pattern: Vec<u8>
}

impl BndmConfig {
    /// Creates a new `BndmConfig` instance.
    ///
    /// # Arguments
    ///
    /// * `search_pattern` - The pattern to search for in the text.
    /// * `wildcard` - An optional wildcard character. If provided, this character in the pattern
    ///                can match any character in the text.
    ///
    /// # Returns
    ///
    /// * `BndmConfig` - A new `BndmConfig` instance.
    ///
    /// # Usage
    ///
    /// Without wildcard:
    ///
    /// ```rust
    /// use bndm::BndmConfig;
    ///
    /// let pattern = b"jumps";
    /// let wildcard = None;
    /// let config = BndmConfig::new(pattern, wildcard);
    /// ```
    ///
    /// With wildcard:
    ///
    /// ```rust
    /// use bndm::BndmConfig;
    ///
    /// let pattern = b"ju??s";
    /// let wildcard = b'?';
    /// let config = BndmConfig::new(pattern, Some(wildcard));
    /// ```
    pub fn new(search_pattern: &[u8], wildcard: Option<u8>) -> BndmConfig {
        let len = get_pattern_length_within_cpu_word(search_pattern.len());

        BndmConfig {
            masks: generate_masks(&search_pattern[..len], wildcard),
            wildcard,
            pattern: search_pattern.to_owned()
        }
    }
}

/// Searches for the pattern in the source string using the BNDM algorithm.
///
/// The function takes a source string and a `BndmConfig` as input. The `BndmConfig`
/// contains the preprocessed pattern and the bitmasks. The function returns the index
/// of the first occurrence of the pattern in the text, or `None` if the pattern is not
/// found.
///
/// # Arguments
///
/// * `source` - The source string to search for the pattern.
/// * `config` - The configuration for the BNDM search, which includes the pattern and the
///              bitmasks.
///
/// # Returns
///
/// * `Option<usize>` - Returns the index of the first occurrence of the pattern in the text,
///                     or `None` if the pattern is not found.
///
/// # Usage
///
/// Without wildcard:
///
/// ```rust
/// use bndm::{BndmConfig, find_pattern};
///
/// let source = b"The quick brown fox jumps over the lazy dog";
/// let pattern = b"jumps";
/// let config = BndmConfig::new(pattern, None);
/// let index = find_pattern(source, &config);
/// assert_eq!(index, Some(20));
/// ```
///
/// With wildcard:
///
/// ```rust
/// use bndm::{BndmConfig, find_pattern};
///
/// let source = b"The quick brown fox jumps over the lazy dog";
/// let pattern = b"ju??s";
/// let config = BndmConfig::new(pattern, Some(b'?'));
/// let index = find_pattern(source, &config);
/// assert_eq!(index, Some(20));
/// ```
pub fn find_pattern(source: &[u8], config: &BndmConfig) -> Option<usize> {
    match config.pattern.len() {
        0 => None,
        1 => config.wildcard
            .map_or(false, |w| w == config.pattern[0]).then_some(0)
            .or_else(|| source.iter().position(|&s| s == config.pattern[0])),
        _ => find_pattern_bndm(source, config)
    }
}

fn find_pattern_bndm(source: &[u8], config: &BndmConfig) -> Option<usize> {
    if config.pattern.len() > source.len() {
        return None;
    }

    let len = get_pattern_length_within_cpu_word(config.pattern.len()) - 1;
    let end = source.len() - config.pattern.len();
    let df = 1 << len;
    let mut i = 0;

    while i <= end {
        let mut j = len;
        let mut last = len;

        let mut d = get_mask(source, config, i + j);
        d = (d << 1) & get_mask(source, config, i + j - 1);
        while d != 0 {
            j -= 1;
            if d & df != 0 {
                if j == 0 {
                    if find_remaining(source, config, i + WORD_SIZE_IN_BITS) {
                        return Some(i);
                    }
                    j += 1;
                }
                last = j;
            }
            d = (d << 1) & get_mask(source, config, i + j - 1);
        }

        i += last;
    }
    None
}

fn get_mask(source: &[u8], config: &BndmConfig, index: usize) -> usize {
    unsafe {
        *config.masks.get_unchecked(*source.get_unchecked(index) as usize)
    }
}

/// Checks if the remaining part of the pattern matches the source string.
///
/// This function is used when the pattern is longer than the CPU word size.
/// It checks the remaining part of the pattern (after the first CPU word size characters)
/// against the corresponding part of the source string.
///
/// # Arguments
///
/// * `source` - The source string to search for the pattern.
/// * `config` - The configuration for the BNDM search, which includes the pattern and the
///              wildcard character.
/// * `start_index` - The index in the source string from where the remaining part of the
///                   pattern should be checked.
///
/// # Returns
///
/// * `bool` - Returns `true` if the remaining part of the pattern matches the corresponding part of the source string, `false` otherwise.
fn find_remaining(source: &[u8], config: &BndmConfig, start_index: usize) -> bool {
    config.pattern.iter().skip(WORD_SIZE_IN_BITS).enumerate().all(|(index, &pattern_byte)| unsafe {
        *source.get_unchecked(start_index + index) == pattern_byte || config.wildcard.map_or(false, |w| pattern_byte == w)
    })
}

fn get_pattern_length_within_cpu_word(search_pattern_length: usize) -> usize {
    min(search_pattern_length, WORD_SIZE_IN_BITS)
}

fn calculate_wildcard_mask(search_pattern: &[u8], wildcard: Option<u8>) -> usize {
    wildcard.map_or(0, |wildcard| search_pattern.iter()
        .fold(0, |mask, &pattern_byte| (mask << 1) | (pattern_byte == wildcard) as usize))
}

fn generate_masks(search_pattern: &[u8], wildcard: Option<u8>) -> [usize; MASKS_TABLE_SIZE] {
    let default_mask = calculate_wildcard_mask(search_pattern, wildcard);
    let mut masks = [default_mask; MASKS_TABLE_SIZE];

    search_pattern.iter().rev().enumerate()
        .for_each(|(i, &pattern_byte)| masks[pattern_byte as usize] |= 1 << i);

    masks
}

#[cfg(test)]
#[path = "./bndm_test.rs"]
mod bndm_test;
