// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
          None => 0,
          Some(val) => {
						/**
							val  						-> Rc<RefCell<TreeNode>>
							val. 						-> RefCell<TreeNode>
							val.borrow_mut() 			-> borrow TreeNode as mutable
							val.borrow_mut().left 		-> Option<Rc<RefCell<TreeNode>>>
							val.borrow_mut().left.take() -> will return a new `Option<Rc<RefCell<TreeNode>>>` and previous one will `become None`
						*/
	
            let leftHandNode = val.borrow_mut().left.take();
            let rightHandNode = val.borrow_mut().right.take();

            return i32::max(Self::max_depth(leftHandNode), Self::max_depth(rightHandNode)) + 1;
          }
      }
    }
}
