#![allow(unused_assignments)]
// https://leetcode.com/problems/two-sum/
pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let limit = nums.len();
        for j in 0..limit {
            for i in j + 1..limit {
                if nums[j] == target - nums[i] {
                    return vec![j as i32, i as i32];
                }
            }
        }
        return vec![0, 0];
        }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum([2, 7, 11, 15].to_vec(), 9), [0, 1]);
    }
    #[test]
    fn test_two_sum2() {
        assert_eq!(Solution::two_sum([3, 2, 3].to_vec(), 6), [0, 2]);
    }
}