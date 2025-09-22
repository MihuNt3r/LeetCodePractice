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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        // if let Some(head) = head {
        //     let mut nodes: Vec<ListNode> = vec![];
        //     let mut curr = head;
        //
        //     while let Some(mut curr) = curr.next.take() {
        //         match curr.next {
        //             Some(next) => {
        //                nodes.push(curr.clone());
        //             },
        //             None => { break; }
        //         }
        //     }
        // } else {
        //     return;
        // }
        // Collect all nodes into a Vec
        let mut nodes = Vec::new();
        let mut cur = head.take();
        while let Some(mut node) = cur {
            cur = node.next.take(); // detach next
            nodes.push(node);
        }

        // Re-link nodes in the reorder pattern
        let mut i = 0;
        let mut j = nodes.len().saturating_sub(1);
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;

        while i <= j {
            if i == j {
                tail.next = Some(Box::new(*nodes[i].clone()));
                break;
            }
            tail.next = Some(Box::new(*nodes[i].clone()));
            tail = tail.next.as_mut().unwrap();

            tail.next = Some(Box::new(*nodes[j].clone()));
            tail = tail.next.as_mut().unwrap();

            i += 1;
            j = j.saturating_sub(1);
        }

        *head = dummy.next;
    }
}

fn main() {
    println!("Hello, world!");
}
