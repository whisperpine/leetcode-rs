// 39. Combination Sum
// https://leetcode.com/problems/combination-sum/description/
// Topics: Backtracking.
// Difficulty: Medium.

#[test]
fn test_39_combination_sum() {}

#[derive(Debug)]
pub struct Solution;

// ---------------------------------
// copy to leetcode starts from here
// ---------------------------------

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut current: Vec<i32> = vec![];
        find_combinations(0, target, candidates.as_ref(), &mut current, &mut result);
        result
    }
}

fn find_combinations(
    index: usize,
    target: i32,
    candidates: &[i32],
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if target == 0 {
        result.push(current.to_vec());
        return;
    }

    for i in index..candidates.len() {
        if candidates[i] <= target {
            current.push(candidates[i]);
            find_combinations(i, target - candidates[i], candidates, current, result);
            current.pop();
        }
    }
}
