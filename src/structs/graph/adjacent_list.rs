use std::{collections::LinkedList, fmt::Debug};

pub type Vertice<T> = (T, LinkedList<usize>);

pub struct AdjacentList<T: Clone>(pub Vec<Vertice<T>>);

impl<T: Clone + Debug> AdjacentList<T> {
    pub fn new() -> AdjacentList<T> {
        AdjacentList(Vec::new())
    }
    pub fn insert(&mut self, elem: T) {
        self.0.push((elem, LinkedList::new()))
    }
    pub fn connect(&mut self, from: usize, to: usize) {
        self.0[from].1.push_back(to)
    }
    pub fn dfs(&self, from: usize) -> Vec<T> {
        let mut v: Vec<T> = Vec::new();
        recurse_dfs(&self, &self.0[from], &mut v);
        v
    }
}
fn recurse_dfs<T: Clone + Debug>(al: &AdjacentList<T>, vertice: &Vertice<T>, vector: &mut Vec<T>) {
    vector.push(vertice.0.clone());
    println!("{:?}", vector);
    if vertice.1.is_empty() {
        return;
    }
    let mut iter = vertice.1.iter();
    for i in iter {
        recurse_dfs(al, &al.0[*i], vector);
    }
}

mod test {
    use super::AdjacentList;

    #[test]
    fn new() {
        let al: AdjacentList<usize> = AdjacentList::new();
        assert!(al.0.is_empty())
    }

    #[test]
    fn insert() {
        let mut al: AdjacentList<usize> = AdjacentList::new();
        assert!(al.0.is_empty());
        al.insert(10);
        assert_eq!(al.0[0].0, 10);
        al.insert(20);
        assert_eq!(al.0[1].0, 20);
        al.insert(30);
        assert_eq!(al.0[2].0, 30);
    }

    #[test]
    fn connect() {
        let mut al: AdjacentList<usize> = AdjacentList::new();
        assert!(al.0.is_empty());
        al.insert(0);
        al.insert(1);
        al.insert(2);
        al.connect(1, 0);
        al.connect(1, 2);
        al.connect(2, 0);
        assert!(al.0[0].1.is_empty());
        assert!(!al.0[1].1.is_empty());
        assert!(!al.0[2].1.is_empty())
    }

    #[test]
    fn dfs() {
        let mut al: AdjacentList<usize> = AdjacentList::new();
        al.insert(0);
        al.insert(1);
        al.insert(2);
        al.insert(3);
        al.insert(4);
        al.insert(5);
        al.insert(6);
        al.insert(7);
        al.connect(0, 1);
        al.connect(0, 2);
        al.connect(0, 3);
        al.connect(1, 4);
        al.connect(1, 5);
        al.connect(2, 6);
        al.connect(3, 7);
        for i in &al.0 {
            println!("{:?}", i);
        }
        let dfsv = al.dfs(0);
        assert_eq!(dfsv[0], 0);
        assert_eq!(dfsv[1], 1);
        assert_eq!(dfsv[2], 4);
        assert_eq!(dfsv[3], 5);
        assert_eq!(dfsv[4], 2);
        assert_eq!(dfsv[5], 6);
        assert_eq!(dfsv[6], 3);
        assert_eq!(dfsv[7], 7);
    }
}
