// 347. Top K Frequent Elements
// https://leetcode.com/problems/top-k-frequent-elements/description/
// Topics: Arrays & Hashing.
// Difficulty: Medium.

#[test]
fn test_347_top_k_frequent_elements() {}

#[derive(Debug)]
pub struct Solution;

// ---------------------------------
// copy to leetcode starts from here
// ---------------------------------

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::cmp::Reverse;
        use std::collections::HashMap;

        let mut tracker: HashMap<i32, usize> = HashMap::new();
        for item in nums {
            tracker.entry(item).and_modify(|n| *n += 1).or_insert(1);
        }
        let mut tuple_vec: Vec<(i32, usize)> = tracker.into_iter().collect();
        // tuple_vec.sort_by(|a, b| b.1.cmp(&a.1));
        tuple_vec.sort_by_key(|item| Reverse(item.1));
        tuple_vec
            .iter()
            .take(k as usize)
            .map(|(num, _)| *num)
            .collect()
    }
}
