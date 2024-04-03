#[derive(Debug)]
pub struct AdjacentMatrix<T>(Vec<(T, Vec<bool>)>);

impl<T> AdjacentMatrix<T> {
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
}

mod test {
    use super::AdjacentMatrix;

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
        println!("GRAPH###########");
        for i in 0..am.0.len() {
            println!("{:?}", am.0[i])
        }
    }
}
