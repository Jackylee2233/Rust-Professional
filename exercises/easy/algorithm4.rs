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

    // 向BST中插入值
    fn insert(&mut self, value: T) {
        match &mut self.root {
            None => self.root = Some(Box::new(TreeNode::new(value))), // 如果根节点为空，创建新节点
            Some(node) => node.insert(value), // 否则委托给TreeNode的insert方法
        }
    }

    // 在BST中搜索值
    fn search(&self, value: T) -> bool {
        // 内部递归搜索函数
        fn inner_search<T: Ord>(node: &Option<Box<TreeNode<T>>>, value: &T) -> bool {
            match node {
                None => false, // 节点为空，未找到值
                Some(node) => {
                    match value.cmp(&node.value) {
                        Ordering::Equal => true,    // 找到相等的值
                        Ordering::Less => inner_search(&node.left, value),   // 在左子树中继续搜索
                        Ordering::Greater => inner_search(&node.right, value), // 在右子树中继续搜索
                    }
                }
            }
        }
        inner_search(&self.root, &value)
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // 在节点中插入值
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                match &mut self.left {
                    None => self.left = Some(Box::new(TreeNode::new(value))), // 左子节点为空时创建新节点
                    Some(node) => node.insert(value), // 递归插入左子树
                }
            }
            Ordering::Greater => {
                match &mut self.right {
                    None => self.right = Some(Box::new(TreeNode::new(value))), // 右子节点为空时创建新节点
                    Some(node) => node.insert(value), // 递归插入右子树
                }
            }
            Ordering::Equal => {} // 忽略重复值
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
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}


