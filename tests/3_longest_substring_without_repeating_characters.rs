// 3. Longest Substring Without Repeating Characters
// https://leetcode.com/problems/longest-substring-without-repeating-characters/description/
// Topics: Sliding Window.
// Difficulty: Medium.

#[test]
fn test_3_longest_substring_without_repeating_characters() {}

#[derive(Debug)]
pub struct Solution;

// ---------------------------------
// copy to leetcode starts from here
// ---------------------------------

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;

        let mut last_index: HashMap<char, usize> = HashMap::new();
        let mut max_length = 0;
        let mut start = 0;

        for (end, ch) in s.chars().enumerate() {
            start = start.max(*last_index.get(&ch).unwrap_or(&0));
            max_length = max_length.max(end - start + 1);
            *last_index.entry(ch).or_default() = end + 1;
        }

        max_length as i32
    }
}
