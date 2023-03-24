use std::{collections::HashMap, usize};

fn main() {}

struct Solution;

impl Solution {
    // 1. two_sum
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        let mut result: Vec<i32> = Vec::with_capacity(2);
        for (index, value) in nums.iter().enumerate() {
            let result_index = map.get(&(target - value));
            match result_index {
                None => {
                    map.insert(*value, index);
                }
                Some(&i) => result = vec![index as i32, i as i32],
            }
        }
        result
    }

    // 7. reverse
    pub fn reverse(x: i32) -> i32 {
        let mut res: i32 = 0;
        if x == 0 {
            0;
        }
        let mut tmp_x = x;
        while tmp_x != 0 {
            let tmp = tmp_x % 10;
            let mul_result = res.checked_mul(10);
            match mul_result {
                None => {
                    res = 0;
                }
                Some(mul) => {
                    let add_result = mul.checked_add(tmp);
                    match add_result {
                        None => {
                            res = 0;
                        }
                        Some(add) => res = add,
                    }
                }
            }
            tmp_x /= 10;
        }
        res
    }

    // 9. is_palindrome
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut res: i32 = 0;
        let mut tmp_x = x;
        while tmp_x != 0 {
            let tmp = tmp_x % 10;
            let mul_result = res.checked_mul(10);
            match mul_result {
                None => {
                    res = 0;
                }
                Some(mul) => {
                    let add_result = mul.checked_add(tmp);
                    match add_result {
                        None => {
                            res = 0;
                        }
                        Some(add) => res = add,
                    }
                }
            }
            tmp_x /= 10;
        }
        res == x
    }
}
