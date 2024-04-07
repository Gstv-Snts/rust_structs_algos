use std::fmt::Debug;

use crate::structs::ll::Queue;

#[derive(Debug)]
pub struct AdjacentMatrix<T>(pub Vec<(T, Vec<bool>)>);

impl<T: Debug + Clone> AdjacentMatrix<T> {
    pub fn new() -> AdjacentMatrix<T> {
        AdjacentMatrix(Vec::new())
    }

    pub fn insert(&mut self, elem: T) {
        let mut v = (elem, vec![]);
        for _ in 0..self.0.len() {
            v.1.push(false);
        }
        self.0.push(v);
        for i in &mut self.0 {
            i.1.push(false);
        }
    }

    pub fn connect(&mut self, from: usize, to: usize) {
        self.0[from].1[to] = true;
    }

    pub fn disconnect(&mut self, from: usize, to: usize) {
        self.0[from].1[to] = false;
    }

    pub fn bfs(&mut self, start: usize) -> Vec<T> {
        let mut q = Queue::new();
        let mut visited = Vec::new();
        q.enqueue(start);
        while !q.is_empty() {
            println!("queue: {:?}", q);
            println!("visited: {:?}", visited);
            let curr = q.dequeue().unwrap();
            visited.push(self.0[curr].0.clone());
            for i in 0..self.0[curr].1.len() {
                if self.0[curr].1[i] {
                    q.enqueue(i)
                }
            }
        }
        visited
    }
}

mod test {
    use crate::structs::graph::adjascent_matrix::AdjacentMatrix;

    #[test]
    fn new() {
        let am: AdjacentMatrix<usize> = AdjacentMatrix::new();
        assert_eq!(am.0.len(), 0);
    }

    #[test]
    fn insert() {
        let mut am = AdjacentMatrix::new();
        am.insert(10);
        assert_eq!(am.0[0], (10, vec![false]));
        am.insert(20);
        assert_eq!(am.0[1], (20, vec![false, false]));
    }

    #[test]
    fn connect() {
        let mut am = AdjacentMatrix::new();
        am.insert(10);
        am.insert(20);
        am.connect(0, 1);
        assert_eq!(am.0[0].1[1], true);
        am.insert(30);
        am.insert(40);
        am.connect(3, 0);
        assert_eq!(am.0[3].1[0], true);
        println!("GRAPH###########");
        for i in 0..am.0.len() {
            println!("{:?}", am.0[i])
        }
    }

    #[test]
    fn disconnect() {
        let mut am = AdjacentMatrix::new();
        am.insert(10);
        am.insert(20);
        am.connect(0, 1);
        assert_eq!(am.0[0].1[1], true);
        am.disconnect(0, 1);
        assert_eq!(am.0[0].1[1], false);
        am.insert(30);
        am.insert(40);
        am.connect(3, 0);
        assert_eq!(am.0[3].1[0], true);
        am.disconnect(3, 0);
        assert_eq!(am.0[3].1[0], false);
    }

    #[test]
    fn bfs() {
        let mut am: AdjacentMatrix<usize> = AdjacentMatrix::new();
        am.insert(0);
        am.insert(1);
        am.insert(2);
        am.insert(3);
        am.insert(4);
        am.insert(5);
        am.insert(6);
        am.insert(7);
        am.connect(0, 1);
        am.connect(0, 2);
        am.connect(0, 3);
        am.connect(1, 4);
        am.connect(1, 5);
        am.connect(2, 6);
        am.connect(3, 7);
        let bfs = am.bfs(0);
        assert_eq!(bfs[0], 0);
        assert_eq!(bfs[1], 1);
        assert_eq!(bfs[2], 2);
        assert_eq!(bfs[3], 3);
        assert_eq!(bfs[4], 4);
        assert_eq!(bfs[5], 5);
        assert_eq!(bfs[6], 6);
        assert_eq!(bfs[7], 7);
    }
}
