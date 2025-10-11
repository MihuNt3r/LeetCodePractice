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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn height(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let n = n.borrow();
                    let left_h = height(&n.left);
                    let right_h = height(&n.right);

                    // if any subtree is unbalanced, propagate -1 upwards
                    if left_h == -1 || right_h == -1 || (left_h - right_h).abs() > 1 {
                        return -1;
                    }

                    left_h.max(right_h) + 1
                }
            }
        }

        height(&root) != -1
    }
}

fn main() {
    println!("Hello, world!");
}
