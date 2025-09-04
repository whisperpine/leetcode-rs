// 55. Jump Game
// https://leetcode.com/problems/jump-game/description/
// Topics: Greedy.
// Difficulty: Medium.

#[test]
fn test_55_jump_game() {}

#[derive(Debug)]
pub struct Solution;

// ---------------------------------
// copy to leetcode starts from here
// ---------------------------------

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_reach: usize = 0;
        for (i, &item) in nums.iter().enumerate() {
            if i > max_reach {
                return false;
            }
            max_reach = max_reach.max(i + item as usize);
            if max_reach >= nums.len() - 1 {
                return true;
            }
        }
        false
    }
}
