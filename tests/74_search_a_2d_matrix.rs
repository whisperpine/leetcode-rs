// 74. Search a 2D Matrix
// https://leetcode.com/problems/search-a-2d-matrix/description/
// Topics: Binary Search.
// Difficulty: Medium.

#[test]
fn test_74_search_a_2d_matrix() {}

#[derive(Debug)]
pub struct Solution;

// ---------------------------------
// copy to leetcode starts from here
// ---------------------------------

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        use std::collections::HashSet;
        let flattened: HashSet<&i32> = HashSet::from_iter(matrix.iter().flatten());
        flattened.contains(&target)
    }
}
