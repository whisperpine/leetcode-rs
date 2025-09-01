// 704. Binary Search
// https://leetcode.com/problems/binary-search/description/
// Topics: Binary Search.
// Difficulty: Easy.

#[test]
fn test_704_binary_search() {}

#[derive(Debug)]
pub struct Solution;

// ---------------------------------
// copy to leetcode starts from here
// ---------------------------------

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // HashMap is even better in this case,
        // because "All the integers in nums are `unique`".
        use std::collections::BTreeMap;
        let nums_map: BTreeMap<i32, usize> =
            nums.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        if let Some(&index) = nums_map.get(&target) {
            index as i32
        } else {
            -1
        }
    }
}
