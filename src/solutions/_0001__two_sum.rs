// 1. Two Sum, Easy
// https://leetcode.com/problems/two-sum/

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for outer in nums.iter().enumerate() {
            let outer_idx = outer.0;
            let outer_val = outer.1;
            for inner in nums[1 + outer_idx..nums.len()].iter().enumerate() {
                let inner_idx = 1 + outer_idx + inner.0;
                let inner_val = inner.1;

                if outer_val + inner_val == target {
                    return vec![outer_idx as i32, inner_idx as i32]
                }
            }
        }
        panic!("Didn't find a solution!")
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test__two_sum__case_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test__two_sum__case_2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test__two_sum__case_3() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
