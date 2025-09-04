// 200. Number of Islands
// https://leetcode.com/problems/number-of-islands/description/
// Topics: Graphs.
// Difficulty: Medium.

#[test]
fn test_200_number_of_islands() {}

#[derive(Debug)]
pub struct Solution;

// ---------------------------------
// copy to leetcode starts from here
// ---------------------------------

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut count: i32 = 0;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '1' {
                    dfs(&mut grid, i, j);
                    count += 1;
                }
            }
        }

        count
    }
}

fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
    if i >= grid.len() || j >= grid[0].len() {
        return;
    }

    if grid[i][j] != '1' {
        return;
    }

    grid[i][j] = '0';

    dfs(grid, i + 1, j);
    dfs(grid, i.wrapping_sub(1), j);
    dfs(grid, i, j + 1);
    dfs(grid, i, j.wrapping_sub(1));
}
