use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
pub struct TreeNode<T> {
    pub value: T,
    pub children: RefCell<Vec<Rc<TreeNode<T>>>>,
    pub parent: RefCell<Weak<TreeNode<T>>>
}

impl<T> TreeNode<T> {
    pub fn new(
                value: T,
                children: RefCell<Vec<Rc<TreeNode<T>>>>
              ) -> Self {
        TreeNode {
            value,
            children,
            parent: RefCell::new(Weak::new()),
        }
    }

    pub fn level(&self) -> usize {
        match self.parent.borrow().upgrade() {
            Some(parent) => 1 + parent.level(),
            None => 0
        }
    }

    // pub fn add_child(&mut self, mut child: Box<TreeNode<T>>) -> () {
    //     self.children.push(child);
    //     let immutable_self = *self;
    //     let weak = Weak::new(); //::from(immutable_self);
    //     child.parent = Some(weak); //Weak::from(*self));
    // }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tree() {
        let a = Rc::new(TreeNode::new('a', RefCell::new(vec![])));
        let b = Rc::new(TreeNode::new('b', RefCell::new(vec![])));
        let c = Rc::new(TreeNode::new('c', RefCell::new(vec![Rc::clone(&a), Rc::clone(&b)])));
        let d = Rc::new(TreeNode::new('d', RefCell::new(vec![])));
        let e = Rc::new(TreeNode::new('e', RefCell::new(vec![Rc::clone(&c), Rc::clone(&d)])));
        
        println!("{:?}", e);

        assert_eq!(e.level(), 0); // Top level
        assert_eq!(d.level(), 1); // Child of e
        // assert_eq!(c.level(), 1); // Child of e
        // assert_eq!(b.level(), 2); // Child of c
        // assert_eq!(a.level(), 2); // Child of c
    }
}