use std::fmt::Debug;
#[derive(Debug)]
struct Node<T: Copy + Clone> {
    data: Option<T>,
    childs: [Option<Box<Node<T>>>; 2],
}
impl<T: Copy + Debug> Node<T> {
    pub fn new() -> Node<T> {
        Node {
            data: None,
            childs: [None, None],
        }
    }
}
#[derive(Debug)]
pub struct BTree<T: Copy + Debug> {
    root: Option<Box<Node<T>>>,
}

impl<T: Copy + Ord + Debug> BTree<T> {
    pub fn new() -> BTree<T> {
        BTree { root: None }
    }
    pub fn print_all(&self) {
        match &self.root {
            Some(root) => {
                print_recurse(root);
            }
            None => {}
        }
    }
    pub fn insert(&mut self, value: T) {
        if self.root.is_none() {
            self.root = Some(Box::new(Node {
                data: Some(value),
                childs: [None, None],
            }));
            return;
        }
        match &mut self.root {
            Some(root) => insert_recurse(root, value),
            None => {}
        }
    }
}
fn print_recurse<T: Copy + Debug + Ord>(node: &Box<Node<T>>) {
    println!("{:?}", node.data);
    if node.childs[0].is_some() {
        match &node.childs[0] {
            Some(child) => print_recurse(child),
            None => {}
        }
    }
    if node.childs[1].is_some() {
        match &node.childs[1] {
            Some(child) => print_recurse(child),
            None => {}
        }
    }
}
fn insert_recurse<T: Copy + Debug + Ord>(node: &mut Box<Node<T>>, value: T) {
    if Some(value) <= node.data {
        if node.childs[0].is_none() {
            node.childs[0] = Some(Box::new(Node {
                data: Some(value),
                childs: [None, None],
            }));
            return;
        } else {
            match &mut node.childs[0] {
                Some(child) => insert_recurse(child, value),
                None => {}
            }
        }
    } else {
        if node.childs[1].is_none() {
            node.childs[1] = Some(Box::new(Node {
                data: Some(value),
                childs: [None, None],
            }));
            return;
        } else {
            match &mut node.childs[1] {
                Some(child) => insert_recurse(child, value),
                None => {}
            }
        }
    }
}
