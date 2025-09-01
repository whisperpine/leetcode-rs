<!-- markdownlint-disable MD033 -->
# LeetCode Rust

An ongoing list of LeetCode problem solutions in Rust.

## NeetCode Roadmap

Refer to [NeetCode Roadmap](https://neetcode.io/roadmap).

<details><summary>Roadmap Mermaid Flowchart</summary>

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
  %% heap("Heap")
  heap("Heap / Priority Queue")
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

</details>

## Get Started

Solutions for LeetCode problems are written in files under [./tests](./tests/)
respectively. This means that rust compiler can check errors and run test cases
for a specific LeetCode problem.
But for comprehensive tests, you should copy the code to LeetCode live editor
and submit. See [LeetCode problem set](https://leetcode.com/problemset/).

There are several ways to run tests:

```sh
# Run an integration test with "cargo test".
cargo test --test TEST -- --no-capture
# Run an integration test with "cargo nextest".
cargo nextest run --test TEST --no-capture
# The same as "cargo nextest" above, but more handy (recommended).
just tf TEST

# For example, run tests in "./tests/15_3sum.rs".
just tf 15_3sum
```

It's recommended to install [just](https://github.com/casey/just)
and [nextest](https://github.com/nextest-rs/nextest),
so that you can use `just tf TEST` command to test.

## Table of Problems

### Array & Hash

| Problem | Difficulty | Solution |
| - | - | - |
| 1. Two Sum | Easy | [1_two_sum.rs](./tests/1_two_sum.rs) |
| 217. Contains Duplicate | Easy | [217_contains_duplicate.rs](./tests/217_contains_duplicate.rs) |
| 242. Valid Anagram | Easy | [242_valid_anagram.rs](./tests/242_valid_anagram.rs) |
| 49. Group Anagrams | Medium | [49_group_anagrams.rs](./tests/49_group_anagrams.rs) |
| 347. Top K Frequent Elements | Medium | [347_top_k_frequent_elements.rs](./tests/347_top_k_frequent_elements.rs) |

### Two Pointers

| Problem | Difficulty | Solution |
| - | - | - |
| 125. Valid Palindrome | Easy | [125_valid_palindrome.rs](./tests/125_valid_palindrome.rs) |
| 15. 3Sum | Medium | [15_3sum.rs](./tests/15_3sum.rs) |

### Stack

| Problem | Difficulty | Solution |
| - | - | - |
| 20. Valid Parentheses | Easy | [20_valid_parentheses.rs](./tests/20_valid_parentheses.rs) |

### Binary Search

| Problem | Difficulty | Solution |
| - | - | - |
| 704. Binary Search | Easy | [704_binary_search.rs](./tests/704_binary_search.rs) |
