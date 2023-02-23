use std::rc::Rc;

pub struct TreeNode<T> {
    pub value: T,
    pub children: Vec<Box<TreeNode<T>>>,
    pub parent: Option<Rc<TreeNode<T>>>
}

impl<T> TreeNode<T> {
    pub fn new(
                value: T,
                children: Vec<Box<TreeNode<T>>>
              ) -> Self {
        TreeNode {
            value,
            children,
            parent: None,
        }
    }
}

pub struct Tree<T> {
    root: Option<TreeNode<T>>
}

impl<T> Tree<T> {
    pub fn new(root: Option<TreeNode<T>>) -> Self {
        Tree {
            root
        }
    }

    pub fn iter(&self) -> PreorderIter<T> {
        PreorderIter::new(self.root.as_ref())
    }
}

pub struct PreorderIter<'a, T> {
    stack: Vec<&'a TreeNode<T>>
}

impl<'a, T> PreorderIter<'a, T> {
    pub fn new<'b: 'a>(root: Option<&'b TreeNode<T>>) -> Self {
        if let Some(node) = root {
            PreorderIter {
                stack: vec![node]
            }
        } else {
            PreorderIter {
                stack: vec![]
            }
        }
    }
}

impl<'a, T> Iterator for PreorderIter<'a, T> {
  type Item = &'a TreeNode<T>;
  fn next(&mut self) -> Option<Self::Item> {
    if let Some(node) = self.stack.pop() {
        let mut item_iter = node.children.iter().peekable();
        if let Some(child) = item_iter.next() {
            self.stack.push(&child)
        }
        return Some(node)
    }
    return None
  }
}

#[cfg(test)]
mod tests {
    use super::{Tree, TreeNode};

    #[test]
    fn test_create_tree() {
        let a = TreeNode::new(4, vec![]);
        let b = TreeNode::new(5, vec![]);
        let c = TreeNode::new(2, vec![Box::from(a), Box::from(b)]);
        
        let d = TreeNode::new(3, vec![]);
        let e = TreeNode::new(1, vec![Box::from(c), Box::from(d)]);
        
        let tree = Tree::new(Some(e));
        
        for _node in tree.iter() {
            // _node.value *= 10;
        }
        
        let mut iterator = tree.iter();
        while let Some(node) = iterator.next() { // equivalent to the for loop construction
            println!("{}", node.value)
        }
    }
}