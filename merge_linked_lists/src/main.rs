fn main() {
    println!("Hello, world!");
}

// Definition for singly-linked list.
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

struct Solution;

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1.clone(), list2.clone()) {
            (None, _) => list2,
            (_, None) => list1,
            (Some(mut l1), Some(mut l2)) => {
                if l1.val < l2.val {
                    l1.next = Solution::merge_two_lists(l1.next, list2);
                    Some(l1)
                } else {
                    l2.next = Solution::merge_two_lists(list1, l2.next);
                    Some(l2)
                }
            },
        }
    }
}