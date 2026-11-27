//! Benchmarks for the BNDM implementation.
//!
//! Run all benchmarks:
//!
//! ```text
//! cargo bench
//! ```
//!
//! Compare a change against a saved baseline:
//!
//! ```text
//! cargo bench -- --save-baseline before   # on the old code
//! cargo bench -- --baseline before        # on the new code, prints the change per benchmark
//! ```
//!
//! Run a subset by passing a filter, e.g. `cargo bench -- find/text`.
//!
//! The search loop is sensitive to code alignment, so an unrelated change can shift some
//! results significantly. To check whether a difference is real, compare both versions with
//! `RUSTFLAGS="-C llvm-args=-align-loops=64"`. On CPUs with performance and efficiency cores,
//! pin the benchmarks to a performance core (e.g. `taskset -c 2` on Linux, `start /affinity 4`
//! on Windows).
//!
//! All haystacks are generated deterministically, so results are comparable between runs.
//! The searched pattern only occurs once, at the very end of the haystack, so every
//! `find/*` benchmark measures a full scan of the haystack.

use std::hint::black_box;
use std::time::Duration;

use bndm::{BndmConfig, find_pattern};
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

const HAYSTACK_SIZE: usize = 256 * 1024;
const PATTERN_LENGTHS: [usize; 8] = [2, 4, 8, 16, 32, 64, 65, 128];
const WILDCARD: u8 = b'?';

const WORDS: [&str; 32] = [
    "the", "quick", "brown", "fox", "jumps", "over", "lazy", "dog", "lorem", "ipsum", "dolor",
    "sit", "amet", "consectetur", "adipiscing", "elit", "sed", "do", "eiusmod", "tempor",
    "incididunt", "ut", "labore", "et", "dolore", "magna", "aliqua", "a", "of", "and", "to", "in",
];

/// Small deterministic PRNG (xorshift64*), so no extra dependencies are needed.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

fn text_haystack(rng: &mut Rng, size: usize) -> Vec<u8> {
    let mut text = Vec::with_capacity(size + 16);
    while text.len() < size {
        text.extend_from_slice(WORDS[rng.below(WORDS.len())].as_bytes());
        text.push(match rng.below(16) {
            0 => b',',
            1 => b'.',
            _ => b' ',
        });
    }
    text.truncate(size);
    text
}

fn binary_haystack(rng: &mut Rng, size: usize) -> Vec<u8> {
    (0..size).map(|_| (rng.next() >> 56) as u8).collect()
}

/// Picks a pattern from the haystack (so it consists of realistic data) and turns every
/// third byte into a wildcard when requested.
fn make_pattern(rng: &mut Rng, haystack: &[u8], len: usize, wildcard: Option<u8>) -> Vec<u8> {
    let start = rng.below(haystack.len() - len);
    let mut pattern = haystack[start..start + len].to_vec();
    if let Some(w) = wildcard {
        pattern.iter_mut().skip(1).step_by(3).for_each(|b| *b = w);
    }
    pattern
}

fn matches_at(haystack: &[u8], pattern: &[u8], wildcard: Option<u8>, pos: usize) -> bool {
    pattern.iter().zip(&haystack[pos..]).all(|(&p, &h)| p == h || Some(p) == wildcard)
}

/// Builds a haystack in which `pattern` only occurs once: at the very end.
fn haystack_with_pattern_at_end(mut haystack: Vec<u8>, pattern: &[u8], wildcard: Option<u8>) -> Vec<u8> {
    let m = pattern.len();
    let mut pos = 0;
    while pos + m <= haystack.len() {
        if matches_at(&haystack, pattern, wildcard, pos) {
            // Break the occurrence by altering the last non-wildcard byte, then re-check
            // from a position where this could have created a new occurrence.
            let k = pattern.iter().rposition(|&p| Some(p) != wildcard).unwrap();
            haystack[pos + k] = haystack[pos + k].wrapping_add(1);
            pos = pos.saturating_sub(m);
        } else {
            pos += 1;
        }
    }
    haystack.extend_from_slice(pattern);
    haystack
}

fn bench_find(c: &mut Criterion, name: &str, base: &[u8], wildcard: Option<u8>, seed: u64) {
    let mut group = c.benchmark_group(format!("find/{name}"));
    let mut rng = Rng(seed);

    for len in PATTERN_LENGTHS {
        let pattern = make_pattern(&mut rng, base, len, wildcard);
        let haystack = haystack_with_pattern_at_end(base.to_vec(), &pattern, wildcard);
        let config = BndmConfig::new(&pattern, wildcard);
        let expected = Some(haystack.len() - len);
        assert_eq!(find_pattern(&haystack, &config), expected, "{name}, pattern length {len}");

        group.throughput(Throughput::Bytes(haystack.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(len), &haystack, |b, haystack| {
            b.iter(|| find_pattern(black_box(haystack), black_box(&config)))
        });
    }
    group.finish();
}

fn bench_find_text(c: &mut Criterion) {
    let base = text_haystack(&mut Rng(0x5EED_0001), HAYSTACK_SIZE);
    bench_find(c, "text", &base, None, 1);
    bench_find(c, "text_wildcard", &base, Some(WILDCARD), 2);
}

fn bench_find_binary(c: &mut Criterion) {
    let base = binary_haystack(&mut Rng(0x5EED_0002), HAYSTACK_SIZE);
    bench_find(c, "binary", &base, None, 3);
    bench_find(c, "binary_wildcard", &base, Some(0x00), 4);
}

fn bench_find_special(c: &mut Criterion) {
    let mut group = c.benchmark_group("find/special");

    // Single byte patterns use a separate code path.
    let base = text_haystack(&mut Rng(0x5EED_0003), HAYSTACK_SIZE);
    let haystack = haystack_with_pattern_at_end(base, b"X", None);
    let config = BndmConfig::new(b"X", None);
    assert_eq!(find_pattern(&haystack, &config), Some(haystack.len() - 1));
    group.throughput(Throughput::Bytes(haystack.len() as u64));
    group.bench_function("single_byte", |b| {
        b.iter(|| find_pattern(black_box(&haystack), black_box(&config)))
    });

    // Patterns starting with wildcards prevent large shifts.
    let base = text_haystack(&mut Rng(0x5EED_0004), HAYSTACK_SIZE);
    let pattern = b"????consectetur adipiscing";
    let haystack = haystack_with_pattern_at_end(base, pattern, Some(WILDCARD));
    let config = BndmConfig::new(pattern, Some(WILDCARD));
    assert_eq!(find_pattern(&haystack, &config), Some(haystack.len() - pattern.len()));
    group.throughput(Throughput::Bytes(haystack.len() as u64));
    group.bench_function("leading_wildcards", |b| {
        b.iter(|| find_pattern(black_box(&haystack), black_box(&config)))
    });

    // Worst case: every window matches almost completely ("aaa...a" vs "aaa...ab").
    for len in [8, 64, 128] {
        let mut pattern = vec![b'a'; len];
        pattern[len - 1] = b'b';
        let mut haystack = vec![b'a'; HAYSTACK_SIZE];
        haystack.extend_from_slice(&pattern);
        let config = BndmConfig::new(&pattern, None);
        assert_eq!(find_pattern(&haystack, &config), Some(HAYSTACK_SIZE));
        group.throughput(Throughput::Bytes(haystack.len() as u64));
        group.bench_with_input(BenchmarkId::new("worst_case", len), &haystack, |b, haystack| {
            b.iter(|| find_pattern(black_box(haystack), black_box(&config)))
        });
    }

    // Many matches: the search is restarted after every hit, like scanning all occurrences.
    let haystack = text_haystack(&mut Rng(0x5EED_0005), 64 * 1024);
    let config = BndmConfig::new(b"the ", None);
    group.throughput(Throughput::Bytes(haystack.len() as u64));
    group.bench_function("find_all_occurrences", |b| {
        b.iter(|| {
            let haystack = black_box(&haystack[..]);
            let mut count = 0;
            let mut pos = 0;
            while let Some(i) = find_pattern(&haystack[pos..], black_box(&config)) {
                count += 1;
                pos += i + 1;
            }
            count
        })
    });

    group.finish();
}

fn bench_config_new(c: &mut Criterion) {
    let mut group = c.benchmark_group("config_new");
    let base = text_haystack(&mut Rng(0x5EED_0006), 4096);

    for len in [4, 64, 128] {
        let pattern = &base[100..100 + len];
        group.bench_with_input(BenchmarkId::new("plain", len), pattern, |b, pattern| {
            b.iter(|| BndmConfig::new(black_box(pattern), None))
        });

        let mut pattern_wild = pattern.to_vec();
        pattern_wild.iter_mut().skip(1).step_by(3).for_each(|b| *b = WILDCARD);
        group.bench_with_input(BenchmarkId::new("wildcard", len), &pattern_wild, |b, pattern| {
            b.iter(|| BndmConfig::new(black_box(pattern), Some(WILDCARD)))
        });
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(2));
    targets = bench_find_text, bench_find_binary, bench_find_special, bench_config_new
}
criterion_main!(benches);
