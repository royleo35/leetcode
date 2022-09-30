pub struct Solution;

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
impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(root) = root {
            let node = root.borrow();
            return match (&node.left, &node.right) {
                (left @ Some(_), None) => format!("{}({})", node.val, Self::tree2str(left.clone())),
                (left, right @ Some(_))  => format!("{}({})({})", node.val, Self::tree2str(left.clone()), Self::tree2str(right.clone())),
                _ => node.val.to_string(),
            }
        }
        String::new()

    }
}