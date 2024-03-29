use core::num;
use std::{hash::Hash, collections::HashMap};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
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
        // (BigUint::parse_bytes(a.as_bytes(), 2).unwrap() + BigUint::parse_bytes(b.as_bytes(), 2).unwrap()).to_str_radix(2)

        let mut u8s = [a, b].iter().map(|x| {
            (0..x.len())
                .step_by(7)
                .map(|i| u8::from_str_radix(&x[x.len().checked_sub(i+7).unwrap_or(0)..x.len()-i], 2).unwrap())
                .collect::<Vec<u8>>()
        }).collect::<Vec<Vec<u8>>>();
        
        u8s.sort_by(|a, b| b.len().cmp(&a.len()));

        let (mut a, b) = (u8s[0].to_owned(), u8s[1].to_owned());

        let u31_max = u32::MAX >> 1;
        println!("{:?}", u31_max);

        let _ = b.iter()
            .enumerate()
            .map(|(i, x)| {
                a[i] += x;
                if a[i] > 127 {
                    a[i] -= 128;
                    if i+1 >= a.len() {
                        a.push(0);
                    }
                    a[i+1] += 1;
                }
            })
            .count();

        let result: String = a.iter()
            .rev()
            .map(|x| format!("{:07b}", x))
            .collect::<Vec<String>>().join("").trim_start_matches('0').into();
        
        if result.len() == 0 {
            "0".into()
        } else {
            result
        }
    }

    pub fn two_sum_bak(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut idx_1, mut idx_2) = (0, numbers.len()-1);

        while idx_1 < idx_2 {
            let sum = numbers[idx_1] + numbers[idx_2];
            if sum == target {
                return vec![idx_1 as i32, idx_2 as i32];
            }

            if sum < target {
                idx_1 += 1;
            } else {
                idx_2 -= 1;
            }
        }

        vec![idx_1 as i32, idx_2 as i32]
    }

    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let (mut a, mut b) = (cost[0], cost[1]);

        for v in cost.into_iter().skip(2) {
            let tmp = b;
            b = std::cmp::min(a, b) + v;
            a = tmp;
        }

        std::cmp::min(a, b)
    }

    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let (mut ptr, mut left, mut right) = (0, 0, nums.iter().skip(1).sum::<i32>());

        while ptr < nums.len() {
            match (left, right) {
                _ if left == right => return ptr as i32,
                _ if ptr + 1 == nums.len() => break,
                _ => {
                    left += nums[ptr];
                    ptr += 1;
                    right -= nums[ptr];
                },
            }
        }

        -1
    }

    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s.chars().filter(|&x| x.is_alphanumeric()).map(|x| x.to_ascii_lowercase()).collect();

        let (mut left, mut right) = (0, chars.len());

        while left < right {
            if chars[left] != chars[right-1] {
                return false;
            }

            left += 1;
            right -= 1;
        }

        true
    }

    pub fn valid_palindrome(s: String) -> bool {
        let chars = s.chars().collect::<Vec<char>>();
        Self::valid_palindrome_inner(&chars[..], 1)
    }

    pub fn valid_palindrome_inner(slice: &[char], tolerance_times: usize) -> bool {
        let (mut l, mut r) = (0, slice.len());

            while l < r {
                match (slice[l], slice[r-1]) {
                    (a, b) if a == b => {
                        l += 1;
                        r -= 1;
                    },
                    _ => {
                        if tolerance_times == 0 {
                            return false;
                        }
                        return Self::valid_palindrome_inner(&slice[l+1..r], tolerance_times-1)
                            || Self::valid_palindrome_inner(&slice[l..r-1], tolerance_times-1);
                    },
                }
            }

            true
    }

    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut result = 0_i32;

        for i in 0..32 {
            let mut sum = 0_i32;
            for num in nums.iter() {
                sum += (num >> i) & 1;
            }
            result |= (sum % 3) << i;
        }
        
        result
    }

    pub fn max_product(words: Vec<String>) -> i32 {
        let have_common = |a: &String, b: &String| -> bool {
            let mut a_bin = 0;
            for v in a.bytes() {
                a_bin |= 1 << (v - 'a' as u8);
            }

            let mut b_bin = 0;
            for v in b.bytes() {
                b_bin |= 1 << (v - 'a' as u8);
            }

            a_bin & b_bin > 0
        };

        let mut max_result = 0;
        for a in words.iter() {
            for b in words.iter() {
                if !have_common(a, b) {
                    max_result = std::cmp::max(max_result, a.len()*b.len());
                }
            }
        }
        max_result as i32
    }

    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        if nums.len() < 3 {
            return result;
        }

        let mut sorted_nums = nums.clone();
        sorted_nums.sort_unstable();

        let mut last_one = sorted_nums[0]-1;
        for idx in 0..(sorted_nums.len()-2) {
            if sorted_nums[idx] == last_one {
                continue;
            }
            last_one = sorted_nums[idx];

            let (mut left, mut right) = (idx + 1, sorted_nums.len()-1);
            while left < right {
                match last_one + sorted_nums[left] + sorted_nums[right] {
                    n if n > 0 => right -= 1,
                    n if n < 0 => left += 1,
                    _ => {
                        result.push(vec![last_one, sorted_nums[left], sorted_nums[right]]);

                        let the_left = sorted_nums[left];
                        while left < sorted_nums.len() && sorted_nums[left] == the_left {
                            left += 1;
                        }

                        let the_right = sorted_nums[right];
                        while right > idx && sorted_nums[right] == the_right {
                            right -= 1;
                        }
                    },
                }
            }
        }

        result
    }

    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        if nums.len() == 0 || nums.iter().sum::<i32>() < target {
            return 0;
        }

        let mut result = nums.len();

        let (mut left, mut right) = (0, 0);
        let mut sum = nums[left..=right].iter().sum::<i32>();
        while left <= right && right < nums.len() {
            if sum >= target {
                result = std::cmp::min(result, right - left + 1);
                sum -= nums[left];
                left += 1;
            } else {
                if right + 1 == nums.len() {
                    break;
                }

                right += 1;
                sum += nums[right];
            }
        }

        result as i32
    }

    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut mp = std::collections::HashMap::new();
        mp.insert(0, 1);

        let (mut sum, mut result) = (0, 0);
        for i in 0..nums.len() {
            sum += nums[i];
            result += mp.get(&(sum-k)).unwrap_or(&0);
            mp.insert(sum, mp.get(&sum).unwrap_or(&0)+1);
        }

        result
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
        assert_eq!(Solution::add_binary("10111111111".into(), "010101".into()).as_str(), "11000010100");
    }

    #[test]
    fn test_pivot_index() {
        assert_eq!(Solution::pivot_index(vec![1, 2, 3, 4, 5]), -1);
    }

    #[test]
    fn test_single_number() {
        assert_eq!(Solution::single_number(vec![2, 2, 2, 3]), 3);
        assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
    }

    #[test]
    fn test_max_product() {
        assert_eq!(Solution::max_product(vec!["eae","ea","aaf","bda","fcf","dc","ac","ce","cefde","dabae"].iter().map(|&x| String::from(x)).collect::<Vec<String>>()), 15);
    }

    #[test]
    fn test_three_sum() {
        assert_eq!(Solution::three_sum(vec![-1,0,1,2,-1,-4]), vec![vec![-1,-1,2], vec![-1, 0, 1]]);
    }

    #[test]
    fn test_min_sub_array_len() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2,3,1,2,4,3]), 2);
        assert_eq!(Solution::min_sub_array_len(4, vec![1,4,4]), 1);
    }
}
