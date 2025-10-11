// 1. Two Sum
// https://leetcode.com/problems/two-sum/description/
// Topics: Arrays & Hashing.
// Difficulty: Easy.

#[rstest::rstest]
#[case(vec![2,7,11,15], 9, vec![0,1])]
#[case(vec![3,2,4], 6, vec![1,2])]
#[case(vec![3,3], 6, vec![0,1])]
fn test_1_two_sum(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: Vec<i32>) {
    let output = Solution::two_sum(nums, target);
    assert_eq!(output, expected);
}

#[derive(Debug)]
pub struct Solution;

// ---------------------------------
// copy to leetcode starts from here
// ---------------------------------

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut prev_map: HashMap<i32, usize> = HashMap::new();
        for (i, item) in nums.iter().enumerate() {
            let diff = target - item;
            if prev_map.contains_key(&diff) {
                return vec![*prev_map.get(&diff).unwrap() as i32, i as i32];
            }
            prev_map.insert(*item, i);
        }
        unreachable!()
    }
}
