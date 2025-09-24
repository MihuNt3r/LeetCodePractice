use std::rc::Rc;
use std::cell::RefCell;

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

struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node_rc) = root.clone() {
            let mut node = node_rc.borrow_mut();

            // Swap left and right
            let left = node.left.take();
            let right = node.right.take();
            node.left = Solution::invert_tree(right);
            node.right = Solution::invert_tree(left);
        }
        root
    }
}

fn invert_tree_recursive(root: Option<Rc<RefCell<TreeNode>>>) {
    if let Some(root) = root {
        let mut node = root.borrow_mut();

        if let Some(left) = node.left.take() {

        }
    }
}

fn main() {
    println!("Hello, world!");
}
