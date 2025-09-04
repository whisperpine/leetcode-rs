// 695. Max Area of Island
// https://leetcode.com/problems/max-area-of-island/description/
// Topics: Graphs.
// Difficulty: Medium.

#[test]
fn test_() {}

#[derive(Debug)]
pub struct Solution;

// ---------------------------------
// copy to leetcode starts from here
// ---------------------------------

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut max_area: i32 = 0;
        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                if grid[x][y] == 1 {
                    max_area = max_area.max(find(&mut grid, x, y));
                }
            }
        }
        max_area
    }
}

fn find(grid: &mut [Vec<i32>], x: usize, y: usize) -> i32 {
    if x >= grid.len() || y >= grid[0].len() {
        return 0;
    }
    if grid[x][y] != 1 {
        return 0;
    }
    grid[x][y] = 0;

    let v1 = find(grid, x + 1, y);
    let v2 = find(grid, x, y + 1);
    let v3 = find(grid, x.wrapping_sub(1), y);
    let v4 = find(grid, x, y.wrapping_sub(1));

    v1 + v2 + v3 + v4 + 1
}
