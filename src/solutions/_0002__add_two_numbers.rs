// 2. Add Two Numbers
// https://leetcode.com/problems/add-two-numbers/

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

    fn from_vec(vals: Vec<i32>) -> Self {
        if vals.len() > 1 {
            return ListNode{
                val: vals[0],
                next: Some(Box::new(
                    Self::from_vec(vals[1..].to_owned())
                )),
            }
        }
        ListNode{
            val: vals[0],
            next: None,
        }
    }
}

impl Solution {
    fn unpack(opt: &Option<Box<ListNode>>) -> (i32, &Option<Box<ListNode>>) {
        match opt {
            Some(ln) => (ln.val, &ln.next),
            None => (0, &None),
        }
    }
    fn add(l1: &Option<Box<ListNode>>, l2: &Option<Box<ListNode>>, rem: i32) -> Option<Box<ListNode>> {
        println!("Calling `add` with l1: {l1:?}, l2: {l2:?}, rem: {rem}");
        if (*l1 == None) & (*l2 == None) & (rem == 0) {
            return None
        }

        let (l1_val, l1_next) = Self::unpack(&l1);
        let (l2_val, l2_next) = Self::unpack(&l2);
        let total = rem + l1_val + l2_val;
        let (div, modulo) = (total / 10, total % 10);
        Some(Box::new(ListNode{
            val: modulo,
            next: Self::add(l1_next, l2_next, div),
        }))
    }

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::add(&l1, &l2, 0)
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test__list_node__from_vec() {
        assert_eq!(
            ListNode::from_vec(vec![2, 4, 3]),
            ListNode{
                val: 2,
                next: Some(Box::new(ListNode{
                    val: 4,
                    next: Some(Box::new(ListNode{
                        val: 3,
                        next: None,
                    })),
                })),
            }
        );
    }

    #[test]
    fn test__add_two_numbers__case_1() {
        assert_eq!(
            Solution::add_two_numbers(
                Some(Box::new(ListNode::from_vec(vec![2, 4, 3]))),
                Some(Box::new(ListNode::from_vec(vec![5, 6, 4]))),
            ),
            Some(Box::new(ListNode::from_vec(vec![7, 0, 8]))),
        );
    }

    #[test]
    fn test__add_two_numbers__case_2() {
        assert_eq!(
            Solution::add_two_numbers(
                Some(Box::new(ListNode::from_vec(vec![0]))),
                Some(Box::new(ListNode::from_vec(vec![0]))),
            ),
            Some(Box::new(ListNode::from_vec(vec![0]))),
        );
    }

    #[test]
    fn test__add_two_numbers__case_3() {
        assert_eq!(
            Solution::add_two_numbers(
                Some(Box::new(ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9]))),
                Some(Box::new(ListNode::from_vec(vec![9, 9, 9, 9]))),
            ),
            Some(Box::new(ListNode::from_vec(vec![8, 9, 9, 9, 0, 0, 0, 1]))),
        );
    }
}
