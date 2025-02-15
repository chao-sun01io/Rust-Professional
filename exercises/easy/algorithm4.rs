/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;

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
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        // method1:
        // Self::insert_sub_tree(&mut self.root, value)

        if let Some(ref mut node) = self.root {
            node.insert(value);
        } else {
            self.root = Some(Box::new(TreeNode::new(value)));
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        // method1:
        // Self::search_sub_tree(&self.root,value)
        
        if let Some(ref node) = self.root {
            node.search(value)
        } else {
            false
        }
    }

    // helper function to insert subtree
    fn insert_sub_tree(tree_node:&mut Option<Box<TreeNode<T>>>, value: T) {
        if let Some(node) = tree_node {
            if  node.value < value {
                Self::insert_sub_tree(&mut node.left, value);
            }else if node.value > value{
                Self::insert_sub_tree(&mut node.right,value);
            }// else:  equal do nothing

            
        } else {
            *tree_node = Some(Box::new(TreeNode::new(value)));
        }
    }

    // helper function to search subtree
    fn search_sub_tree(tree_node: &Option<Box<TreeNode<T>>>, value: T)-> bool {
        if let Some(node) = tree_node {
            match node.value.cmp(&value) {
                Ordering::Equal => true,
                Ordering::Greater => Self::search_sub_tree(&node.right, value),
                Ordering::Less => Self::search_sub_tree(&node.left, value)
            }
        }else{
           false
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => match self.left {
                Some(ref mut node) => node.insert(value),
                None => self.left = Some(Box::new(TreeNode::new(value))),
            },
            Ordering::Greater => match self.right {
                Some(ref mut node) => node.insert(value),
                None => self.right = Some(Box::new(TreeNode::new(value))),
            },
            Ordering::Equal => {}
        }
    }

    fn search(&self, value: T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Equal => true,
            Ordering::Less => match self.left {
                Some(ref node) => node.search(value),
                None => false,
            },
            Ordering::Greater => match self.right {
                Some(ref node) => node.search(value),
                None => false,
            },
        }
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
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
