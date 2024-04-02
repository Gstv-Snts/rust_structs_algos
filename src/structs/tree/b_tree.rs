use std::{fmt::Debug, ptr::null_mut};

pub struct BTree<T: Copy + Debug> {
    root: Link<T>,
}

type Link<T> = *mut Node<T>;

struct Node<T: Copy> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Copy + Ord + Debug> BTree<T> {
    pub fn new() -> BTree<T> {
        BTree { root: null_mut() }
    }

    pub fn insert(&mut self, elem: T) {
        if self.root.is_null() {
            self.root = Box::into_raw(Box::new(Node {
                elem,
                left: null_mut(),
                right: null_mut(),
            }));
            return;
        }
        insert_recurse(self.root, elem)
    }
}

fn insert_recurse<T: Copy + PartialOrd>(node: Link<T>, elem: T) {
    if elem <= unsafe { (*node).elem } {
        if unsafe { (*node).left.is_null() } {
            unsafe {
                (*node).left = Box::into_raw(Box::new(Node {
                    elem,
                    left: null_mut(),
                    right: null_mut(),
                }));
            }
        } else {
            insert_recurse(unsafe { (*node).left }, elem)
        }
    } else {
        if unsafe { (*node).right.is_null() } {
            unsafe {
                (*node).right = Box::into_raw(Box::new(Node {
                    elem,
                    left: null_mut(),
                    right: null_mut(),
                }));
            }
        } else {
            insert_recurse(unsafe { (*node).right }, elem)
        }
    }
}

#[cfg(test)]
mod test {
    use super::BTree;

    #[test]
    fn new() {
        let bt: BTree<usize> = BTree::new();
        assert!(bt.root.is_null())
    }

    #[test]
    fn insert() {
        let mut bt: BTree<usize> = BTree::new();
        assert!(bt.root.is_null());
        bt.insert(10);
        assert!(!bt.root.is_null());
        bt.insert(5);
        assert_eq!(unsafe { (*(*bt.root).left).elem }, 5);
        bt.insert(15);
        assert_eq!(unsafe { (*(*bt.root).right).elem }, 15)
    }
}
