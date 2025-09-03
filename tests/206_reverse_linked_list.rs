// 206. Reverse Linked List
// https://leetcode.com/problems/reverse-linked-list/description/
// Topics: Linked List.
// Difficulty: Easy.

#[test]
fn test_206_reverse_linked_list() {}

#[derive(Debug)]
pub struct Solution;

// ---------------------------------
// copy to leetcode starts from here
// ---------------------------------

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;
        let mut curr: Option<Box<ListNode>> = head;

        while let Some(mut boxed_node) = curr {
            curr = boxed_node.next;
            boxed_node.next = prev;
            prev = Some(boxed_node);
        }

        prev
    }
}
