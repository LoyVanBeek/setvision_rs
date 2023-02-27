use std::fmt::Display;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
pub struct TreeNode<T: Display> {
    pub value: T,
    pub children: RefCell<Vec<Rc<TreeNode<T>>>>,
    pub parent: RefCell<Weak<TreeNode<T>>>
}

impl<T: Display> TreeNode<T> {
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
        println!("I have value {:}", self.value);
        match self.parent.borrow().upgrade() {
            Some(parent) => {
                println!("My parent has value {:}", parent.value);
                1 + parent.level()
            },
            None => 0
        }
    }

    pub fn add_child(self: Rc<TreeNode<T>>, mut child: Rc<TreeNode<T>>) -> () {
        self.children.borrow_mut().push(Rc::clone(&child));
        // let immutable_self = *self;
        // let weak = Weak::new(); //::from(immutable_self);
        // child.parent = RefCell::new(Weak::clone(&self));
        *child.parent.borrow_mut() = Rc::downgrade(&self);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tree() {
        let a = Rc::new(TreeNode::new('a', RefCell::new(vec![])));
        let b = Rc::new(TreeNode::new('b', RefCell::new(vec![])));
        // let c = Rc::new(TreeNode::new('c', RefCell::new(vec![Rc::clone(&a), Rc::clone(&b)])));
        let c = Rc::new(TreeNode::new('c', RefCell::new(vec![])));
        // c.add_child(a);
        // c.add_child(b);
        let d = Rc::new(TreeNode::new('d', RefCell::new(vec![])));
        // let e = Rc::new(TreeNode::new('e', RefCell::new(vec![Rc::clone(&c), Rc::clone(&d)])));
        let e = Rc::new(TreeNode::new('e', RefCell::new(vec![])));
        
        println!("{:?}", e);

        assert_eq!(e.level(), 0); // Top level
        assert_eq!(d.level(), 1); // Child of e
        // assert_eq!(c.level(), 1); // Child of e
        // assert_eq!(b.level(), 2); // Child of c
        // assert_eq!(a.level(), 2); // Child of c
    }
}