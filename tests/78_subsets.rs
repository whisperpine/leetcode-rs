// 78. Subsets
// https://leetcode.com/problems/subsets/description/
// Topics: Backtracking.
// Difficulty: Medium.

#[test]
fn test_78_subsets() {}

#[derive(Debug)]
pub struct Solution;

// ---------------------------------
// copy to leetcode starts from here
// ---------------------------------

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let result: Vec<Vec<i32>> = vec![vec![]];
        recursion(result, nums).0
    }
}

fn recursion(mut v: Vec<Vec<i32>>, mut nums_left: Vec<i32>) -> (Vec<Vec<i32>>, Vec<i32>) {
    let Some(num) = nums_left.pop() else {
        return (v, nums_left);
    };
    let mut new_v = vec![];
    for item in v.iter() {
        let mut item = item.clone();
        item.push(num);
        new_v.push(item);
    }
    v.append(&mut new_v);
    recursion(v, nums_left)
}
