# LeetCode Rust

LeetCode problem solutions in Rust.

## References

- [NeetCode Roadmap](https://neetcode.io/roadmap).
- [LeetCode problem set](https://leetcode.com/problemset/).

## NeetCode Roadmap

```mermaid
flowchart LR
  array_hash("Array & Hash")
  two_pointers("Two Pointers")
  stack("Stack")
  binary_search("Binary Search")
  sliding_window("Sliding Window")
  linked_list("Linked List")
  trees("Trees")
  tries("Tries")
  backtracing("Backtracing")
  heap("Heap / Priority Queue")
  %% heap("Heap")
  graphs("Graphs")
  %% dp("DP")
  dp("Dynamic Programming")
  bit("Bit Manipulation")
  math("Math & Geometry")
  intervals("Intervals")
  greedy("Greedy")
  advanced_graphs("Advanced Graphs")

  array_hash --> two_pointers
  array_hash --> stack

  two_pointers --> binary_search
  two_pointers --> sliding_window
  two_pointers --> linked_list

  binary_search --> trees
  linked_list --> trees

  trees --> tries
  trees --> backtracing
  trees --> heap

  backtracing --> dp
  backtracing --> graphs

  dp --> bit
  dp --> math
  graphs --> math

  graphs --> advanced_graphs
  heap --> advanced_graphs
  heap --> intervals
  heap --> greedy
```

## Get Started

## Table of Problems
