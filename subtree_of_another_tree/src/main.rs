// Definition for a binary tree node.
use std::rc::Rc;
use std::cell::RefCell;

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
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match sub_root {
            None => true,        // subRoot is null → always true
            Some(_) => match root {
                None => false,   // root is null but subRoot not → false
                Some(node) => {
                    if Self::same_tree(Some(node.clone()), sub_root.clone()) {
                        return true;
                    }
                    Self::is_subtree(node.borrow().left.clone(), sub_root.clone())
                        || Self::is_subtree(node.borrow().right.clone(), sub_root)
                }
            },
        }
    }

    fn same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p), Some(q)) => {
                let pb = p.borrow();
                let qb = q.borrow();
                pb.val == qb.val
                    && Self::same_tree(pb.left.clone(), qb.left.clone())
                    && Self::same_tree(pb.right.clone(), qb.right.clone())
            }
            _ => false,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
