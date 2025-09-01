// 20. Valid Parentheses
// https://leetcode.com/problems/valid-parentheses/description/
// Topics: Stack.
// Difficulty: Easy.

#[test]
fn test_20_valid_parentheses() {}

#[derive(Debug)]
pub struct Solution;

// ---------------------------------
// copy to leetcode starts from here
// ---------------------------------

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut chars: Vec<char> = vec![];
        for c in s.chars() {
            match c {
                '(' => chars.push('('),
                '[' => chars.push('['),
                '{' => chars.push('{'),
                ')' => {
                    if chars.is_empty() {
                        return false;
                    }
                    if let Some(ccc) = chars.pop()
                        && ccc != '('
                    {
                        return false;
                    }
                }
                ']' => {
                    if chars.is_empty() {
                        return false;
                    }
                    if let Some(ccc) = chars.pop()
                        && ccc != '['
                    {
                        return false;
                    }
                }
                '}' => {
                    if chars.is_empty() {
                        return false;
                    }
                    if let Some(ccc) = chars.pop()
                        && ccc != '{'
                    {
                        return false;
                    }
                }
                _ => unreachable!(),
            };
        }

        if !chars.is_empty() {
            return false;
        }

        true
    }
}
