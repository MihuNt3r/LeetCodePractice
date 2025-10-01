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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::dfs(&root, 0, &mut res);
        res
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, depth: usize, res: &mut Vec<Vec<i32>>) {
        if let Some(n) = node {
            let n = n.borrow();
            if res.len() == depth {
                res.push(vec![]);
            }
            res[depth].push(n.val);

            Self::dfs(&n.left, depth + 1, res);
            Self::dfs(&n.right, depth + 1, res);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
