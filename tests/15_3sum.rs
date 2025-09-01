// 15. 3Sum
// https://leetcode.com/problems/3sum/description/
// Topics: Two Pointers.
// Difficulty: Medium.

#[test]
fn test_15_3sum() {}

#[derive(Debug)]
pub struct Solution;

// ---------------------------------
// copy to leetcode starts from here
// ---------------------------------

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];

        let length = nums.len();
        if length < 3 {
            return result;
        }

        nums.sort();

        for i in 0..(length - 2) {
            if nums[i] > 0 {
                break;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = length - 1;
            let target = -nums[i];

            use std::cmp::Ordering;
            while left < right {
                match (nums[left] + nums[right]).cmp(&target) {
                    Ordering::Equal => {
                        result.push(vec![nums[i], nums[left], nums[right]]);

                        // deduplicate
                        while left < right && nums[left] == nums[left + 1] {
                            left += 1;
                        }
                        while left < right && nums[right] == nums[right - 1] {
                            right -= 1;
                        }

                        left += 1;
                        right -= 1;
                    }
                    Ordering::Less => left += 1,
                    Ordering::Greater => right -= 1,
                }
            }
        }

        result
    }
}
