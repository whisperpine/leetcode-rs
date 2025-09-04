// 703. Kth Largest Element in a Stream
// https://leetcode.com/problems/kth-largest-element-in-a-stream/description/
// Topics: Heap / Priority Queue.
// Difficulty: Easy.

#![allow(unused)]

#[test]
fn test_703_kth_largest_element_in_a_stream() {}

#[derive(Debug)]
pub struct Solution;

// ---------------------------------
// copy to leetcode starts from here
// ---------------------------------

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    k: usize,
    nums: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut heap = BinaryHeap::new();
        for n in nums {
            heap.push(Reverse(n));
            if heap.len() > k {
                heap.pop();
            }
        }
        Self { k, nums: heap }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.nums.push(Reverse(val));
        if self.nums.len() > self.k {
            self.nums.pop();
        }
        self.nums.peek().unwrap().0
    }
}

// Your KthLargest object will be instantiated and called as such:
// let obj = KthLargest::new(k, nums);
// let ret_1: i32 = obj.add(val);
