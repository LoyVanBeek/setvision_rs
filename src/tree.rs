pub struct TreeNode<T> {
    pub value: T,
    pub left:  Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>
}

impl<T> TreeNode<T> {
    pub fn new(
                value: T,
                left: Option<Box<TreeNode<T>>>,
                right: Option<Box<TreeNode<T>>>
              ) -> Self {
        TreeNode {
            value,
            left,
            right
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
          if let Some(right) = &node.right {
              self.stack.push(&right)
          }
          if let Some(left) = &node.left {
              self.stack.push(&left)
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
        let a = TreeNode::new(4, None, None);
        let b = TreeNode::new(5, None, None);
        let c = TreeNode::new(2, Some(Box::from(a)), Some(Box::from(b)));
        
        let d = TreeNode::new(3, None, None);
        let e = TreeNode::new(1, Some(Box::from(c)), Some(Box::from(d)));
        
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