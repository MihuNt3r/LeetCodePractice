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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut nodes: Vec<ListNode> = Vec::new();
        let mut curr = head.clone();

        while let Some(mut node) = curr {
            curr = node.next.take();
            nodes.push(*node);
        }

        let remove_index = nodes.len() - n as usize;
        // If we remove the head
        let mut new_head: Option<Box<ListNode>> = None;
        let mut tail = &mut new_head;

        for (i, mut node) in nodes.into_iter().enumerate() {
            if i == remove_index {
                continue; // skip this node
            }
            node.next = None; // detach old links
            *tail = Some(Box::new(node));
            tail = &mut tail.as_mut().unwrap().next;
        }

        new_head
    }
}

// Build linked list from vector
pub fn build_list(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    let mut tail = &mut head;

    for v in values {
        let new_node = Box::new(ListNode::new(v));
        *tail = Some(new_node);
        tail = &mut tail.as_mut().unwrap().next;
    }

    head
}

fn main() {
    let list = build_list(vec![1, 2, 3, 4, 5]);

    Solution::remove_nth_from_end(list, 2);
    // println!("{:#?}", list);
}
