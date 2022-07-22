#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    pub value: i32,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Tree {
    root: Option<Box<Node>>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct BinaryTree {
    root: Option<Box<Node>>,
}

pub trait TreeHandle {
    fn add(&mut self, node: Node);
    fn print(&self);
}

impl Node {
    #[inline]
    pub fn new(value: i32) -> Self {
        Node {
            left: None,
            right: None,
            value,
        }
    }

    pub fn set_left(&mut self, node: Node) {
        self.left = Some(Box::new(node));
    }

    pub fn set_right(&mut self, node: Node) {
        self.right = Some(Box::new(node));
    }
}

impl TreeHandle for Tree {
    fn add(&mut self, node: Node) {
        print!("{:?}", node);
        todo!()
    }

    fn print(&self) {
        match &self.root {
            Some(node) => println!("{}", node.value),
            _ => println!("Node"),
        }
    }
}

impl TreeHandle for BinaryTree {
    fn add(&mut self, node: Node) {
        let root = &mut self.root;

        match root {
            Some(root) => {
                root.left = Some(Box::new(node));
            }
            None => self.root = Some(Box::new(node)),
        }
    }

    fn print(&self) {
        let mut node = &self.root;

        while *node != None {
            match &node {
                Some(vale) => {
                    println!("{}", vale.value);
                    node = &vale.left;
                }
                None => node = &None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tree_demo::{BinaryTree, Node, Tree, TreeHandle};

    #[test]
    fn test_node() {
        let mut node = Node::new(1);
        node.set_left(Node::new(2));
        let tree = Tree {
            root: Some(Box::new(node)),
        };
        tree.print();
    }

    #[test]
    fn test_b_tree() {
        let mut bt = BinaryTree { root: None };
        bt.add(Node::new(1));
        bt.add(Node::new(2));

        bt.print();
    }
}
