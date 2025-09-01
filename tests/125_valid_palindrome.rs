// 125. Valid Palindrome
// https://leetcode.com/problems/valid-palindrome/description/
// Topics: Two Pointers.
// Difficulty: Easy.

#[test]
fn test_125_valid_palindrome() {}

#[derive(Debug)]
pub struct Solution;

// ---------------------------------
// copy to leetcode starts from here
// ---------------------------------

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let processed_chars: Vec<char> = s
            .to_lowercase()
            .chars()
            .filter(|x| x.is_alphanumeric())
            .collect();
        if processed_chars.is_empty() {
            return true;
        }
        let mut left: usize = 0;
        let mut right: usize = processed_chars.len() - 1;
        while left < right {
            if processed_chars[left] != processed_chars[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}
