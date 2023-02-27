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

// pub struct Tree<T> {
//     root: Option<TreeNode<T>>
// }

// impl<T> Tree<T> {
//     pub fn new(root: Option<TreeNode<T>>) -> Self {
//         Tree {
//             root
//         }
//     }

//     pub fn iter(&self) -> PreorderIter<T> {
//         PreorderIter::new(self.root.as_ref())
//     }
// }

// pub struct PreorderIter<'a, T> {
//     stack: Vec<&'a TreeNode<T>>
// }

// impl<'a, T> PreorderIter<'a, T> {
//     pub fn new<'b: 'a>(root: Option<&'b TreeNode<T>>) -> Self {
//         if let Some(node) = root {
//             PreorderIter {
//                 stack: vec![node]
//             }
//         } else {
//             PreorderIter {
//                 stack: vec![]
//             }
//         }
//     }
// }

// impl<'a, T> Iterator for PreorderIter<'a, T> {
//   type Item = &'a TreeNode<T>;
//   fn next(&mut self) -> Option<Self::Item> {
//     if let Some(node) = self.stack.pop() {
//         let mut item_iter = node.children.borrow().iter().peekable();
//         if let Some(child) = item_iter.next() {
//             self.stack.push(&child)
//         }
//         return Some(node)
//     }
//     return None
//   }
// }

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
        // let tree = Tree::new(Some(e));
        
        // for _node in tree.iter() {
        //     // _node.value *= 10;
        // }
        
        // let mut iterator = tree.iter();
        // while let Some(node) = iterator.next() { // equivalent to the for loop construction
        //     println!("{}", node.value)
        // }
    }
}