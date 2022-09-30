mod l606_construct_string_from_binary_tree;
use std::rc::Rc;
use std::cell::RefCell;

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use super::*;

    #[test]
    fn test_cell() {
        let c = Cell::new(5);
        c.set(4);
        let res = c.take();
        assert_eq!(res, 4);
        let res1 = c.take();
        assert_eq!(res1, 0);

        //
        let c1 = 10;
        let ref_c1 = &c1;
        let cc = Cell::new(ref_c1);
        cc.set(&5);
        let r1 = *cc.get();
        assert_eq!(r1, 5);
    }

    #[test]
    fn test_box() {
        let b = Box::new(5);
        println!("{}", b);
    }

    #[test]
    fn tree_to_str() {
        use l606_construct_string_from_binary_tree::TreeNode;
        let t = Some(Rc::new(RefCell::new(TreeNode{
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        let res = l606_construct_string_from_binary_tree::Solution::tree2str(t);
        assert_eq!(res, "1(2(4))(3)".to_string());
    }
}
