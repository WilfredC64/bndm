// Copyright (C) 2019 - 2026 Wilfred Bos
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
//! Unlike the traditional BNDM algorithm, this implementation is optimized by checking the
//! last 2 bytes of each window before entering the inner loop. Additionally, it does not have
//! the limitation that the pattern size is limited to the word size of the CPU. It's possible
//! to search for larger patterns than the word size and still benefit from the performance
//! that BNDM offers.
//!
//! One of the key features of this implementation is its support for wildcard search, for
//! which BNDM is ideally suited. Wildcards are encoded in the bitmasks during preprocessing,
//! so the BNDM scan does exactly the same work per byte with or without wildcards. Where
//! many other search algorithms need extra comparisons or special handling to support
//! wildcards, the BNDM scan supports them at no additional cost per byte.
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
    ///   can match any character in the text.
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
///   bitmasks.
///
/// # Returns
///
/// * `Option<usize>` - Returns the index of the first occurrence of the pattern in the text,
///   or `None` if the pattern is not found.
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
            .is_some_and(|w| w == config.pattern[0] && !source.is_empty()).then_some(0)
            .or_else(|| find_byte(source, config.pattern[0])),
        _ => find_pattern_bndm(source, config)
    }
}

fn find_byte(source: &[u8], byte: u8) -> Option<usize> {
    // Find the chunk containing the byte, then its position in that chunk or in the tail.
    // The fold has no early exit and `as_chunks` gives it exactly 32 bytes on every iteration,
    // which allows it to be vectorized.
    let (chunks, tail) = source.as_chunks::<32>();
    let i = chunks.iter().position(|chunk| {
        chunk.iter().fold(false, |found, &b| found | (b == byte))
    }).unwrap_or(chunks.len());
    let chunk = chunks.get(i).map_or(tail, |chunk| chunk.as_slice());
    chunk.iter().position(|&b| b == byte).map(|j| i * 32 + j)
}

#[inline(never)]
fn find_pattern_bndm(source: &[u8], config: &BndmConfig) -> Option<usize> {
    let pattern_len = config.pattern.len();
    if pattern_len > source.len() {
        return None;
    }

    let masks = &config.masks;
    let len = get_pattern_length_within_cpu_word(pattern_len) - 1;
    let end = source.len() - pattern_len;
    let mask_at = |index: usize| -> usize {
        debug_assert!(index < source.len());
        // SAFETY: every caller uses index <= i + len, and i <= end with end + len < source.len().
        masks[unsafe { *source.get_unchecked(index) } as usize]
    };

    let mut i = 0;
    while i <= end {
        let mut d = (mask_at(i + len) << 1) & mask_at(i + len - 1);
        let mut j = len;

        if d != 0 {
            j = len - 1;
            loop {
                if j == 0 {
                    if find_remaining(source, config, i + WORD_SIZE_IN_BITS) {
                        return Some(i);
                    }
                    j = 1;
                    break;
                }
                d = (d << 1) & mask_at(i + j - 1);
                if d == 0 {
                    break;
                }
                j -= 1;
            }
        }

        i += j;
    }
    None
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
///   wildcard character.
/// * `start_index` - The index in the source string from where the remaining part of the
///   pattern should be checked.
///
/// # Returns
///
/// * `bool` - Returns `true` if the remaining part of the pattern matches the corresponding part of the source string, `false` otherwise.
fn find_remaining(source: &[u8], config: &BndmConfig, start_index: usize) -> bool {
    let Some(remaining) = config.pattern.get(WORD_SIZE_IN_BITS..) else {
        return true;
    };
    let source = &source[start_index..start_index + remaining.len()];

    match config.wildcard {
        None => source == remaining,
        Some(wildcard) => remaining.iter().zip(source)
            .all(|(&pattern_byte, &source_byte)| pattern_byte == source_byte || pattern_byte == wildcard)
    }
}

fn get_pattern_length_within_cpu_word(search_pattern_length: usize) -> usize {
    min(search_pattern_length, WORD_SIZE_IN_BITS)
}

fn calculate_wildcard_mask(search_pattern: &[u8], wildcard: Option<u8>) -> usize {
    wildcard.map_or(0, |wildcard| search_pattern.iter()
        .fold(0, |mask, &pattern_byte| (mask << 1) | (pattern_byte == wildcard) as usize))
}

fn generate_masks(search_pattern: &[u8], wildcard: Option<u8>) -> [usize; MASKS_TABLE_SIZE] {
    let mut masks = [calculate_wildcard_mask(search_pattern, wildcard); MASKS_TABLE_SIZE];
    let mut bit = 1usize;

    for &byte in search_pattern.iter().rev() {
        masks[byte as usize] |= bit;
        bit <<= 1;
    }

    masks
}

#[cfg(test)]
#[path = "./bndm_test.rs"]
mod bndm_test;
