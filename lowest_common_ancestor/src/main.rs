// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match (root, p, q) {
            (Some(root_node), Some(p_node), Some(q_node)) => {
                let root_val = root_node.borrow().val;
                let p_val = p_node.borrow().val;
                let q_val = q_node.borrow().val;

                if p_val.max(q_val) < root_val {
                    // search in the left subtree
                    Solution::lowest_common_ancestor(root_node.borrow().left.clone(), Some(p_node), Some(q_node))
                } else if p_val.min(q_val) > root_val {
                    // search in the right subtree
                    Solution::lowest_common_ancestor(root_node.borrow().right.clone(), Some(p_node), Some(q_node))
                } else {
                    // current node is the LCA
                    Some(root_node)
                }
            }
            _ => None,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
