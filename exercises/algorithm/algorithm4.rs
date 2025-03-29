/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

//I AM DONE
use std::cmp::Ordering;
use std::fmt::{Debug, Display};


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord+Display,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        let Some(node) = &mut self.root else {
            self.root = Some(Box::new(TreeNode::new(value)));
            return;
        };
        node.insert(value);
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        let mut finded = false;
        let mut mid = &self.root;
        while let Some(node) = mid {
            if node.value == value {
                finded = true;
                break;
            } else if value < node.value {
                mid = &node.left;
            } else {
                mid = &node.right;
            }
        }
        finded
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        let mut mid;

        match value.cmp(&self.value) {
            Ordering::Less => {
                if let None = self.left {
                    self.left = Some(Box::new(TreeNode::new(value)));
                    return;
                } else {
                    mid = &mut self.left;
                }
            },
            Ordering::Equal => {
                return;
            },
            Ordering::Greater => {
                if let None = self.right {
                    self.right = Some(Box::new(TreeNode::new(value)));
                    return;
                } else {
                    mid = &mut self.right;
                }
            },
        }
        
        while let Some(node) = mid {
            if value < node.value {
                mid = &mut node.left;
            } else {
                mid = &mut node.right;
            }
        }
        *mid = Some(Box::new(TreeNode::new(value)));
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


