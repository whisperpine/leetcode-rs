// 217. Contains Duplicate
// https://leetcode.com/problems/contains-duplicate/description/
// Topics: Arrays & Hashing.
// Difficulty: Easy.

#[test]
fn test_217_contains_duplicate() {}

#[derive(Debug)]
pub struct Solution;

// ---------------------------------
// copy to leetcode starts from here
// ---------------------------------

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        let mut set: HashSet<i32> = HashSet::new();
        !nums.iter().all(|&n| set.insert(n))
    }
}

// // Another solution (easier to read at first glance).
// impl Solution {
//     pub fn contains_duplicate(nums: Vec<i32>) -> bool {
//         use std::collections::HashSet;
//         let mut prev_set: HashSet<i32> = HashSet::new();
//         for item in nums {
//             if prev_set.contains(&item) {
//                 return true;
//             }
//             prev_set.insert(item);
//         }
//         false
//     }
// }
