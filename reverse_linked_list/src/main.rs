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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut prev, mut curr): (Option<Box<ListNode>>, Option<Box<ListNode>>) = (None, head);

        while let Some(mut curr_node) = curr.take() {
            let temp = curr_node.next;
            curr_node.next = prev.take();
            prev = Some(curr_node);
            curr = temp;
        }

        prev
    }
}

fn main() {
    let node5 = Box::new(ListNode::new(5));
    let node4 = Box::new(ListNode { val: 4, next: Some(node5) });
    let node3 = Box::new(ListNode { val: 3, next: Some(node4) });
    let node2 = Box::new(ListNode { val: 2, next: Some(node3) });
    let head = Some(Box::new(ListNode { val: 1, next: Some(node2) }));

    // Now `head` is the start of the linked list [1,2,3,4,5]
    println!("{:?}", head);
}


