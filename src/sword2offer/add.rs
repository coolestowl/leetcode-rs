use std::{collections::HashMap, fmt::format, str::FromStr};
use num_bigint::BigUint;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

struct Solution {
}

impl Solution {
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        let mut set = std::collections::HashSet::new();
        for num in nums {
            if set.contains(&num) {
                return num;
            } else {
                set.insert(num);
            }
        }
        return -1;
    }

    pub fn majority_element(nums: Vec<i32>) -> i32 {
        /*
        let mut counter = std::collections::HashMap::new();

        let mut max = 0;
        let mut max_num = 0;

        for num in nums {
            let mut cnt = 1;

            if let Some(&v) = counter.get(&num) {
                cnt = v + 1;
            }

            counter.insert(num, cnt);

            if cnt > max {
                max = cnt;
                max_num = num;
            }
        }
        
        max_num
        */

        let (mut counter, mut the_number) = (0, 0);

        for num in nums {
            if counter == 0 {
                (counter, the_number) = (1, num);
                continue;
            }

            counter += if num == the_number { 1 } else { -1 };
        }

        the_number
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_index = std::collections::HashMap::new();

        let (mut idx_a, mut idx_b) = (0, 0);

        for (idx, &num) in nums.iter().enumerate() {
            let sub = target - num;

            if let Some(&sub_idx) = num_index.get(&sub) {
                idx_a = idx as i32;
                idx_b = sub_idx;
                break;
            }

            num_index.insert(sub, idx as i32);
        }

        vec![idx_a, idx_b]
    }

    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(n1), Some(n2)) => {
                if n1.val < n2.val {
                    Some(Box::new(ListNode{val: n1.val, next: Solution::merge_two_lists(n1.next, Some(n2))}))
                } else {
                    Some(Box::new(ListNode{val: n2.val, next: Solution::merge_two_lists(Some(n1), n2.next)}))
                }
            },
            (Some(n1), None) => Some(n1),
            (None, Some(n2)) => Some(n2),
            _ => None,
        }
    }

    pub fn add_binary(a: String, b: String) -> String {
        (BigUint::parse_bytes(a.as_bytes(), 2).unwrap() + BigUint::parse_bytes(b.as_bytes(), 2).unwrap()).to_str_radix(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_repeat_number() {
        assert_eq!(Solution::find_repeat_number(vec![2, 3, 1, 0, 2, 5, 3]), 2);
    }

    #[test]
    fn test_add_binary() {
        assert_eq!(Solution::add_binary("1010".into(), "1011".into()).as_str(), "10101");
    }
}
