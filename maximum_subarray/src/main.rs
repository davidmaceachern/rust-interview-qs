#![allow(unused_assignments)]
// https://leetcode.com/problems/maximum-subarray

pub struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_current = nums[0];
        let mut max_global = max_current;

        for &i in nums[1..].iter() {
            if max_current < 0 { 
                max_current = i
            } else { 
                max_current += i
            };
            max_global = std::cmp::max(max_global, max_current);
        }
        max_global
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_sub_array1() {
        assert_eq!(Solution::max_sub_array([-2,1,-3,4,-1,2,1,-5,4].to_vec()), 6);
    }

    #[test]
    fn test_max_sub_array2() {
        assert_eq!(Solution::max_sub_array([-2147483647].to_vec()), -2147483647);
    }

    #[test]
    fn test_max_sub_array3() {
        assert_eq!(Solution::max_sub_array([-1].to_vec()), -1);
    }

    #[test]
    fn test_max_sub_array4() {
        assert_eq!(Solution::max_sub_array([0].to_vec()), 0);
    }

    #[test]
    fn test_max_sub_array5() {
        assert_eq!(Solution::max_sub_array([1].to_vec()), 1);
    }
}
