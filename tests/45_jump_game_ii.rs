// 45. Jump Game II
// https://leetcode.com/problems/jump-game-ii/description/
// Topics: Greedy.
// Difficulty: Medium.

#[test]
fn test_45_jump_game_ii() {}

#[derive(Debug)]
pub struct Solution;

// ---------------------------------
// copy to leetcode starts from here
// ---------------------------------

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut jumps = 0;
        let mut current_end = 0;
        let mut max_reach = 0;

        for (i, &item) in nums.iter().enumerate().take(nums.len() - 1) {
            max_reach = max_reach.max(i + item as usize);
            if i == current_end {
                jumps += 1;
                current_end = max_reach;
            }
        }

        jumps
    }
}
