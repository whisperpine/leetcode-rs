// 56. Merge Intervals
// https://leetcode.com/problems/merge-intervals/description/
// Topics: Intervals.
// Difficulty: Medium.

#[test]
fn test_56_merge_intervals() {}

#[derive(Debug)]
pub struct Solution;

// ---------------------------------
// copy to leetcode starts from here
// ---------------------------------

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::BinaryHeap;
        let mut heap: BinaryHeap<StartEnd> = BinaryHeap::new();
        for item in intervals.iter() {
            heap.push(StartEnd {
                start: *item.first().unwrap(),
                end: *item.last().unwrap(),
            });
        }
        let mut result: Vec<Vec<i32>> = vec![];
        while !heap.is_empty() {
            // heap is not empty, thus unwrap() is safe.
            let bigger_end = heap.pop().unwrap();
            let Some(smaller_end) = heap.peek() else {
                result.push(bigger_end.into());
                break;
            };
            // if no overlap
            if smaller_end.end < bigger_end.start {
                result.push(bigger_end.into());
            } else {
                let new_item = StartEnd {
                    start: std::cmp::min(smaller_end.start, bigger_end.start),
                    end: bigger_end.end,
                };
                heap.pop();
                heap.push(new_item);
            }
        }
        result
    }
}

#[derive(Eq)]
struct StartEnd {
    start: i32,
    end: i32,
}

impl From<StartEnd> for Vec<i32> {
    fn from(val: StartEnd) -> Self {
        vec![val.start, val.end]
    }
}

impl PartialEq for StartEnd {
    fn eq(&self, other: &Self) -> bool {
        self.end == other.end
    }
}

impl PartialOrd for StartEnd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StartEnd {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.end.cmp(&other.end)
    }
}
