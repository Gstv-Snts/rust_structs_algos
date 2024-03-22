use std::fmt::Debug;

#[derive(Debug)]
pub struct Heap<T> {
    data: Vec<T>,
    length: usize,
}
impl<T: Debug + PartialOrd + Clone> Heap<T> {
    pub fn new() -> Heap<T> {
        Heap {
            data: Vec::new(),
            length: 0,
        }
    }
    pub fn insert(&mut self, value: T) {
        self.data.push(value);
        self.length += 1;
    }
    pub fn heapify_max(&mut self) {
        for i in 0..self.length {
            println!("{:?}", self.data);
            match self.length {
                1 => break,
                2 => {
                    if self.data[1] < self.data[0] {
                        self.switch(left_child(0), 0)
                    }
                }
                _ => {
                    if left_child(i) < self.length {
                        if &self.data[i] < &self.data[left_child(i)] {
                            self.switch(left_child(i), i);
                        }
                    }
                    if right_child(i) < self.length {
                        if &self.data[i] < &self.data[right_child(i)] {
                            self.switch(right_child(i), i);
                        }
                    }
                }
            }
        }
    }
    pub fn heapify_min(&mut self) {
        for i in 0..self.length {
            println!("{:?}", self.data);
            match self.length {
                1 => break,
                2 => {
                    if self.data[1] > self.data[0] {
                        self.switch(left_child(0), 0)
                    }
                }
                _ => {
                    if left_child(i) < self.length {
                        if &self.data[i] > &self.data[left_child(i)] {
                            self.switch(left_child(i), i);
                        }
                    }
                    if right_child(i) < self.length {
                        if &self.data[i] > &self.data[right_child(i)] {
                            self.switch(right_child(i), i);
                        }
                    }
                }
            }
        }
    }
    fn switch(&mut self, from: usize, to: usize) {
        let tmp = self.data[from].clone();
        self.data[from] = self.data[to].clone();
        self.data[to] = tmp
    }
}
fn left_child(index: usize) -> usize {
    (2 * index) + 1
}
fn right_child(index: usize) -> usize {
    (2 * index) + 2
}
fn parent(index: usize) -> usize {
    (index - 1) / 2
}

#[cfg(test)]
mod test {
    use super::Heap;

    #[test]
    fn heap_max() {
        let mut h = Heap::new();
        h.insert(50);
        h.insert(100);
        h.insert(40);
        h.insert(10);
        h.insert(15);
        h.insert(50);
        h.insert(50);
        h.insert(40);
        h.heapify_max();
        assert_eq!(h.length, 8);
        assert_eq!(h.data[0], 100);
    }
    #[test]
    fn heap_min() {
        let mut h = Heap::new();
        h.insert(50);
        h.insert(100);
        h.insert(40);
        h.insert(10);
        h.insert(15);
        h.insert(50);
        h.insert(50);
        h.insert(40);
        h.heapify_min();
        assert_eq!(h.length, 8);
        assert_eq!(h.data[h.length - 1], 100);
    }
}
