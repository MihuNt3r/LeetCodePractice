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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_mirror(
            t1: Option<Rc<RefCell<TreeNode>>>,
            t2: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (t1, t2) {
                (None, None) => true,
                (Some(n1), Some(n2)) => {
                    let n1 = n1.borrow();
                    let n2 = n2.borrow();
                    n1.val == n2.val
                        && is_mirror(n1.left.clone(), n2.right.clone())
                        && is_mirror(n1.right.clone(), n2.left.clone())
                }
                _ => false,
            }
        }

        if let Some(node) = root {
            let node = node.borrow();
            return is_mirror(node.left.clone(), node.right.clone());
        }

        false
    }
}

fn main() {
    println!("Hello, world!");
}
