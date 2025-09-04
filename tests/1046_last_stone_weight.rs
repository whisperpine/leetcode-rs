// 1046. Last Stone Weight
// https://leetcode.com/problems/last-stone-weight/description/
// Topics: Heap / Priority Queue.
// Difficulty: Easy.

#![allow(unused)]

#[test]
fn test_1046_last_stone_weight() {}

#[derive(Debug)]
pub struct Solution;

// ---------------------------------
// copy to leetcode starts from here
// ---------------------------------

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;
        let mut heap: BinaryHeap<i32> = BinaryHeap::from(stones);
        while heap.len() >= 2 {
            let x = heap.pop().unwrap();
            let y = heap.pop().unwrap();
            let diff = (x - y).abs();
            if diff != 0 {
                heap.push(diff);
            }
        }
        *heap.peek().unwrap_or(&0)
    }
}
