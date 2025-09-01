// 49. Group Anagrams
// https://leetcode.com/problems/group-anagrams/description/
// Topics: Arrays & Hashing.
// Difficulty: Medium.

#[test]
fn test_49_group_anagrams() {}

#[derive(Debug)]
pub struct Solution;

// ---------------------------------
// copy to leetcode starts from here
// ---------------------------------

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;

        const A_AS_USIZE: usize = 'a' as usize;
        const LETTER_SIZE: usize = 26;
        let mut tracker: HashMap<[i32; LETTER_SIZE], Vec<String>> = HashMap::new();
        for str in strs {
            let mut char_tracker: [i32; LETTER_SIZE] = [0; LETTER_SIZE];
            for ch in str.chars() {
                let curr_ind = (ch as usize) - A_AS_USIZE;
                char_tracker[curr_ind] += 1;
            }
            tracker.entry(char_tracker).or_default().push(str);
        }

        let mut result: Vec<Vec<String>> = Vec::new();
        for (_, v) in tracker {
            result.push(v);
        }
        result
    }
}
