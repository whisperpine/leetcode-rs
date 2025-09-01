// 242. Valid Anagram
// https://leetcode.com/problems/valid-anagram/description/
// Topics: Arrays & Hashing.
// Difficulty: Easy.

#[test]
fn test_242_valid_anagram() {
    let s = "cat".to_owned();
    let t = "tac".to_owned();
    let result = Solution::is_anagram(s, t);
    assert!(result);
}

#[derive(Debug)]
pub struct Solution;

// ---------------------------------
// copy to leetcode starts from here
// ---------------------------------

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        use std::collections::HashMap;
        let mut s_char_set: HashMap<char, u32> = HashMap::new();
        for character in s.chars() {
            *s_char_set.entry(character).or_insert(0) += 1;
        }
        let mut t_char_set: HashMap<char, u32> = HashMap::new();
        for character in t.chars() {
            *t_char_set.entry(character).or_insert(0) += 1;
        }
        s_char_set == t_char_set
    }
}

// // Another solution (basically the same, just using different methods)
// impl Solution {
//     pub fn is_anagram(s: String, t: String) -> bool {
//         use std::collections::HashMap;
//         let mut s_char_set: HashMap<char, u32> = HashMap::new();
//         for character in s.chars() {
//             s_char_set
//                 .entry(character)
//                 .and_modify(|n| *n += 1)
//                 .or_insert(1);
//         }
//         let mut t_char_set: HashMap<char, u32> = HashMap::new();
//         for character in t.chars() {
//             t_char_set
//                 .entry(character)
//                 .and_modify(|n| *n += 1)
//                 .or_insert(1);
//         }
//         s_char_set == t_char_set
//     }
// }
