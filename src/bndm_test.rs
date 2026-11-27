use super::*;

#[test]
fn find_pattern_in_middle() {
    let source = b"The quick brown fox jumps over the lazy dog";
    let pattern = b"jumps";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(20));
}

#[test]
fn find_pattern_at_beginning() {
    let source = b"The quick brown fox jumps over the lazy dog";
    let pattern = b"The quick";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(0));
}

#[test]
fn find_pattern_at_end() {
    let source = b"The quick brown fox jumps over the lazy dog";
    let pattern = b"dog";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(40));
}

#[test]
fn find_pattern_no_match() {
    let source = b"The quick brown fox jumps over the lazy dog";
    let pattern = b"cat";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, None);
}

#[test]
fn find_pattern_fully() {
    let source = b"The quick brown fox jumps over the lazy dog";
    let pattern = b"The quick brown fox jumps over the lazy dog";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(0));
}

#[test]
fn find_pattern_partially_end() {
    let source = b"The quick brown fox jumps over the lazy dog";
    let pattern = b"test fox jumps over the lazy";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, None);
}

#[test]
fn find_pattern_partially_beginning() {
    let source = b"The quick brown fox jumps over the lazy dog";
    let pattern = b"fox jumps over the test";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, None);
}

#[test]
fn find_pattern_wildcard_middle() {
    let source = b"The quick brown fox jumps over the lazy dog";
    let pattern = b"j?mps";
    let config = BndmConfig::new(pattern, Some(b'?'));
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(20));
}

#[test]
fn find_pattern_wildcard_end() {
    let source = b"The quick brown fox jumps over the lazy dog";
    let pattern = b"jump?";
    let config = BndmConfig::new(pattern, Some(b'?'));
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(20));
}

#[test]
fn find_pattern_wildcard_beginning() {
    let source = b"The quick brown fox jumps over the lazy dog";
    let pattern = b"?umps";
    let config = BndmConfig::new(pattern, Some(b'?'));
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(20));
}

#[test]
fn find_pattern_wildcard_multiple() {
    let source = b"The quick brown fox jumps over the lazy dog";
    let pattern = b"?um?s";
    let config = BndmConfig::new(pattern, Some(b'?'));
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(20));
}

#[test]
fn find_pattern_wildcards_only() {
    let source = b"The quick brown fox jumps over the lazy dog";
    let pattern = b"??????";
    let config = BndmConfig::new(pattern, Some(b'?'));
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(0));
}

#[test]
fn find_pattern_wildcard_only() {
    let source = b"The quick brown fox jumps over the lazy dog";
    let pattern = b"?";
    let config = BndmConfig::new(pattern, Some(b'?'));
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(0));
}

#[test]
fn find_pattern_wildcard_only_in_empty_source() {
    let source = b"";
    let pattern = b"?";
    let config = BndmConfig::new(pattern, Some(b'?'));
    let index = find_pattern(source, &config);

    assert_eq!(index, None);
}

#[test]
fn find_pattern_wildcard_beginning_first_word() {
    let source = b"The quick brown fox jumps over the lazy dog";
    let pattern = b"??he";
    let config = BndmConfig::new(pattern, Some(b'?'));
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(30));
}

#[test]
fn find_pattern_match_1char() {
    let source = b"The quick brown fox jumps over the lazy dog";
    let pattern = b"q";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(4));
}

#[test]
fn find_pattern_match_0char() {
    let source = b"The quick brown fox jumps over the lazy dog";
    let pattern = b"";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, None);
}

#[test]
fn find_pattern_match_2chars() {
    let source = b"The quick brown fox jumps over the lazy dog";
    let pattern = b"do";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(40));
}

#[test]
fn find_pattern_match_31chars() {
    let source = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua";
    let pattern = b"consectetur adipiscing elit, se";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(28));
}

#[test]
fn find_pattern_match_32chars() {
    let source = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua";
    let pattern = b"consectetur adipiscing elit, sed";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(28));
}

#[test]
fn find_pattern_match_33chars() {
    let source = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua";
    let pattern = b"consectetur adipiscing elit, sed ";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(28));
}

#[test]
fn find_pattern_match_63chars() {
    let source = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua";
    let pattern = b"consectetur adipiscing elit, sed do eiusmod tempor incididunt u";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(28));
}

#[test]
fn find_pattern_match_64chars() {
    let source = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua";
    let pattern = b"consectetur adipiscing elit, sed do eiusmod tempor incididunt ut";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(28));
}

#[test]
fn find_pattern_match_65chars_middle() {
    let source = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua";
    let pattern = b"consectetur adipiscing elit, sed do eiusmod tempor incididunt ut ";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(28));
}

#[test]
fn find_pattern_match_65chars_end() {
    let source = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua";
    let pattern = b"sed do eiusmod tempor incididunt ut labore et dolore magna aliqua";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(57));
}

#[test]
fn find_pattern_match_65chars_beginning() {
    let source = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua";
    let pattern = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do e";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(0));
}

#[test]
fn find_pattern_match_64chars_middle_wildcard_at_end() {
    let source = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua";
    let pattern = b"consectetur adipiscing elit, sed do eiusmod tempor incididunt u!";
    let config = BndmConfig::new(pattern, Some(b'!'));
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(28));
}

#[test]
fn find_pattern_match_65chars_middle_wildcard_at_end() {
    let source = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua";
    let pattern = b"consectetur adipiscing elit, sed do eiusmod tempor incididunt ut!";
    let config = BndmConfig::new(pattern, Some(b'!'));
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(28));
}

#[test]
fn find_pattern_match_65chars_middle_wildcard_second_last() {
    let source = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua";
    let pattern = b"consectetur adipiscing elit, sed do eiusmod tempor incididunt u! ";
    let config = BndmConfig::new(pattern, Some(b'!'));
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(28));
}

#[test]
fn find_pattern_match_66chars_middle_wildcard_second_last() {
    let source = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua";
    let pattern = b"consectetur adipiscing elit, sed do eiusmod tempor incididunt ut!l";
    let config = BndmConfig::new(pattern, Some(b'!'));
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(28));
}

#[test]
fn find_pattern_longer_than_source() {
    let source = b"Lorem ipsum dolor sit amet, consectetur";
    let pattern = b"consectetur adipiscing elit, sed do eiusmod tempor incididunt";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, None);
}

#[test]
fn find_pattern_same_size_source_no_match_at_end() {
    let source = b"Lorem ipsum dolor sit amet, consectetur";
    let pattern = b"Lorem ipsum dolor sit amet, consectetuR";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, None);
}

#[test]
fn find_pattern_same_size_source_wildcard_at_end() {
    let source = b"Lorem ipsum dolor sit amet, consectetur";
    let pattern = b"Lorem ipsum dolor sit amet, consectetu!";
    let config = BndmConfig::new(pattern, Some(b'!'));
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(0));
}

#[test]
fn find_pattern_in_empty_source() {
    let source = b"";
    let pattern = b"consectetur adipiscing elit, sed do eiusmod tempor incididunt";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, None);
}

#[test]
fn find_empty_pattern_in_empty_source() {
    let source = b"";
    let pattern = b"";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, None);
}

#[test]
fn find_pattern_match_large_repeating_char_beginning() {
    let source = b"baaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    let pattern = b"baaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(67));
}

#[test]
fn find_pattern_match_large_repeating_char_at_index_68() {
    let source = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab";
    let pattern = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(68));
}

#[test]
fn find_pattern_match_large_repeating_char_at_index_67() {
    let source = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab";
    let pattern = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(67));
}

#[test]
fn find_pattern_match_large_repeating_char_at_index_66() {
    let source = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab";
    let pattern = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(66));
}

#[test]
fn find_pattern_match_large_repeating_char_at_index_65() {
    let source = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab";
    let pattern = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(65));
}

#[test]
fn find_pattern_match_large_repeating_char_at_index_64() {
    let source = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab";
    let pattern = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(64));
}

#[test]
fn find_pattern_match_large_repeating_char_at_index_63() {
    let source = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab";
    let pattern = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(63));
}

#[test]
fn find_pattern_match_repeating_char_beginning() {
    let source = b"aaaaaaab";
    let pattern = b"aaab";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(4));
}

#[test]
fn find_pattern_match_repeating_char_end() {
    let source = b"baaaaaaabaaaaaaaa";
    let pattern = b"baaaaaaaa";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(8));
}

#[test]
fn find_small_pattern_at_beginning() {
    let source = b"baaaaaaabaaaaaaaa";
    let pattern = b"ba";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(0));
}

#[test]
fn find_small_pattern_at_end() {
    let source = b"baaaaaaabaaaaaaac";
    let pattern = b"ac";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(15));
}

#[test]
fn find_small_pattern_with_wildcard_at_beginning_match_end() {
    let source = b"baaaaaaabaaaaaaac";
    let pattern = b"?c";
    let config = BndmConfig::new(pattern, Some(b'?'));
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(15));
}

#[test]
fn find_small_pattern_with_wildcard_at_beginning_match_beginning() {
    let source = b"baaaaaaabaaaaaaac";
    let pattern = b"?a";
    let config = BndmConfig::new(pattern, Some(b'?'));
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(0));
}

#[test]
fn find_small_pattern_with_wildcard_at_beginning_no_match() {
    let source = b"daaaaaaabaaaaaaac";
    let pattern = b"?d";
    let config = BndmConfig::new(pattern, Some(b'?'));
    let index = find_pattern(source, &config);

    assert_eq!(index, None);
}

#[test]
fn find_small_pattern_with_wildcard_at_end_match_beginning() {
    let source = b"baaaaaaabaaaaaaac";
    let pattern = b"b?";
    let config = BndmConfig::new(pattern, Some(b'?'));
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(0));
}

#[test]
fn find_small_pattern_with_wildcard_at_end_match_end() {
    let source = b"baaaaaaabaaaaaadc";
    let pattern = b"d?";
    let config = BndmConfig::new(pattern, Some(b'?'));
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(15));
}

#[test]
fn find_small_pattern_with_wildcard_no_match() {
    let source = b"baaaaaaabaaaaaadc";
    let pattern = b"c?";
    let config = BndmConfig::new(pattern, Some(b'?'));
    let index = find_pattern(source, &config);

    assert_eq!(index, None);
}

#[test]
fn find_small_pattern_skip_window() {
    let source = b"aaaaaaaaaaaaaaabbbbbbbbabcbbbbbcbbbbbbbbbbbcbbbbbbbbbbbbb";
    let pattern = b"bbbbbbcbbbbbbbbbbbb";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(37));
}

#[test]
fn find_small_pattern_skip_window_no_match() {
    let source = b"baaaaabbbbbbbbbbbaaaaaaaaaaaaaaa";
    let pattern = b"aaaaabbbbbbbbbbbbaaaaa";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, None);
}

#[test]
fn find_small_pattern_skip_window_match() {
    let source = b"baaaaabbbbbbbbbbbaaaaaaabbbbbbbbbbbbaaaaaaacccccaaaaaa";
    let pattern = b"aaaaabbbbbbbbbbbbaaaaa";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(19));
}

#[test]
fn find_small_pattern_skip_window_match_immediate() {
    let source = b"bbbaaaaabbbbbbbbbbbccaaaaabbbbbbbbbbbbccccc";
    let pattern = b"aaaaabbbbbbbbbbbbccccc";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(21));
}

#[test]
fn find_small_pattern_skip_window_match_longest_suffix() {
    let source = b"bbbaaaaabbbbbbbbbbbcccaaaaabbbbbbbbbbbbccccc";
    let pattern = b"aaaaabbbbbbbbbbbbccccc";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(22));
}

#[test]
fn find_small_pattern_skip_window_match_longest_suffix_long_pattern() {
    let source = b"bbbaaaaabbbbbbbbbbbcccaaaaabbbbbbbbbcccccaaaaabbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbcccccaaaaabbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbcccccddd";
    let pattern = b"aaaaabbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbcccccaaaaabbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbccccc";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(41));
}

#[test]
fn find_overlapping_match_after_wildcard_tail_mismatch() {
    let mut pattern = vec![b'a'; WORD_SIZE_IN_BITS];
    pattern.extend_from_slice(b"?b");
    let config = BndmConfig::new(&pattern, Some(b'?'));

    let mut source = vec![b'a'; WORD_SIZE_IN_BITS + 1];
    source.extend_from_slice(b"?b");

    assert_eq!(find_pattern(&source, &config), Some(1));
}

#[test]
fn find_single_byte_at_chunk_boundaries() {
    // The single byte search scans in chunks of 32 bytes, the last chunk is shorter.
    for index in [0, 1, 31, 32, 33, 63, 64, 95, 96, 99] {
        let mut source = vec![b'a'; 100];
        source[index] = b'b';
        let config = BndmConfig::new(b"b", None);
        let found = find_pattern(&source, &config);

        assert_eq!(found, Some(index), "byte at index {index}");
    }
}

#[test]
fn find_single_byte_first_of_multiple_chunks() {
    let mut source = vec![b'a'; 100];
    source[40] = b'b';
    source[70] = b'b';
    let config = BndmConfig::new(b"b", None);
    let index = find_pattern(&source, &config);

    assert_eq!(index, Some(40));
}

#[test]
fn find_single_byte_no_match_in_multiple_chunks() {
    let source = vec![b'a'; 100];
    let config = BndmConfig::new(b"b", None);
    let index = find_pattern(&source, &config);

    assert_eq!(index, None);
}

#[test]
fn find_two_byte_pattern_in_source_of_two_bytes() {
    let config = BndmConfig::new(b"ab", None);

    assert_eq!(find_pattern(b"ab", &config), Some(0));
    assert_eq!(find_pattern(b"ba", &config), None);
    assert_eq!(find_pattern(b"a", &config), None);
}

#[test]
fn find_pattern_returns_first_occurrence() {
    let source = b"XabcXabcXabc";
    let pattern = b"abc";
    let config = BndmConfig::new(pattern, None);
    let index = find_pattern(source, &config);

    assert_eq!(index, Some(1));
}

#[test]
fn find_wildcard_character_is_literal_without_wildcard() {
    let config = BndmConfig::new(b"j?mps", None);
    assert_eq!(find_pattern(b"The quick brown fox jumps over the lazy dog", &config), None);

    let config = BndmConfig::new(b"t? y", None);
    assert_eq!(find_pattern(b"what? yes", &config), Some(3));
}

#[test]
fn find_binary_pattern() {
    let source = [0x10, 0xff, 0x20, 0x00, 0xff, 0x20];
    let pattern = [0x00, 0xff, 0x20];

    let config = BndmConfig::new(&pattern, None);
    assert_eq!(find_pattern(&source, &config), Some(3));

    // With 0x00 as wildcard, the first byte matches anything.
    let config = BndmConfig::new(&pattern, Some(0x00));
    assert_eq!(find_pattern(&source, &config), Some(0));
}

/// Returns 64 distinct bytes, so the first CPU word of a pattern can't partially overlap itself.
fn distinct_word() -> Vec<u8> {
    (0x40..0x80).collect()
}

#[test]
fn find_large_pattern_after_mismatch_in_remaining_part() {
    // The first 64 bytes match at index 0, but the remaining part doesn't.
    let pattern = [distinct_word(), b"tail".to_vec()].concat();
    let source = [distinct_word(), b"tall".to_vec(), distinct_word(), b"tail".to_vec()].concat();
    let config = BndmConfig::new(&pattern, None);
    let index = find_pattern(&source, &config);

    assert_eq!(index, Some(68));
}

#[test]
fn find_large_pattern_with_wildcards_in_remaining_part() {
    let pattern = [distinct_word(), b"t??l".to_vec()].concat();
    let source = [b"xx".to_vec(), distinct_word(), b"tall".to_vec()].concat();
    let config = BndmConfig::new(&pattern, Some(b'?'));
    let index = find_pattern(&source, &config);

    assert_eq!(index, Some(2));
}

#[test]
fn find_large_pattern_returns_first_occurrence() {
    let pattern = [distinct_word(), b"!".to_vec()].concat();
    let source = [b"-".to_vec(), pattern.clone(), pattern.clone()].concat();
    let config = BndmConfig::new(&pattern, None);
    let index = find_pattern(&source, &config);

    assert_eq!(index, Some(1));
}

#[test]
fn find_pattern_spanning_multiple_cpu_words() {
    let pattern: Vec<u8> = (0..200).map(|i| b'a' + (i % 26) as u8).collect();
    let source = [b"0123456789".to_vec(), pattern.clone()].concat();
    let config = BndmConfig::new(&pattern, None);
    let index = find_pattern(&source, &config);

    assert_eq!(index, Some(10));
}

/// Straightforward reference implementation used to verify the optimized search.
fn find_pattern_naive(source: &[u8], pattern: &[u8], wildcard: Option<u8>) -> Option<usize> {
    if pattern.is_empty() || pattern.len() > source.len() {
        return None;
    }
    (0..=source.len() - pattern.len()).find(|&i| {
        pattern.iter().zip(&source[i..]).all(|(&p, &s)| p == s || Some(p) == wildcard)
    })
}

struct Rng(u64);

impl Rng {
    fn below(&mut self, n: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 % n as u64) as usize
    }
}

#[test]
fn find_pattern_matches_naive_search() {
    let mut rng = Rng(0x2545_F491_4F6C_DD1D);
    let wildcard = b'?';

    for iteration in 0..40_000 {
        // Small alphabets cause many partial matches, which exercises all code paths.
        let alphabet = [b"ab".as_slice(), b"abc", b"abcdefgh"][iteration % 3];
        let source_len = rng.below(300);
        let mut source: Vec<u8> = (0..source_len).map(|_| alphabet[rng.below(alphabet.len())]).collect();
        let pattern_len = 1 + rng.below(140);
        let mut pattern: Vec<u8> = if source_len >= pattern_len && rng.below(2) == 0 {
            let start = rng.below(source_len - pattern_len + 1);
            source[start..start + pattern_len].to_vec()
        } else {
            (0..pattern_len).map(|_| alphabet[rng.below(alphabet.len())]).collect()
        };

        let use_wildcard = rng.below(2) == 0;
        if use_wildcard {
            let count = rng.below(pattern_len.min(8) + 1);
            (0..count).for_each(|_| pattern[rng.below(pattern_len)] = wildcard);
            // The wildcard byte in the source must only match a wildcard in the pattern.
            if rng.below(4) == 0 && source_len > 0 {
                source[rng.below(source_len)] = wildcard;
            }
        }
        let wildcard = use_wildcard.then_some(wildcard);

        let config = BndmConfig::new(&pattern, wildcard);
        let expected = find_pattern_naive(&source, &pattern, wildcard);
        assert_eq!(
            find_pattern(&source, &config), expected,
            "source: {:?}, pattern: {:?}, wildcard: {:?}",
            String::from_utf8_lossy(&source), String::from_utf8_lossy(&pattern), wildcard
        );
    }
}
