use std::collections::LinkedList;

#[derive(Debug)]
pub struct Queue<T> {
    element: LinkedList<T>,
}
impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            element: LinkedList::new(),
        }
    }
    pub fn enqueue(&mut self, value: T) {
        self.element.push_back(value)
    }
    pub fn dequeue(&mut self) -> Option<T> {
        self.element.pop_front()
    }
    pub fn peek_front(&mut self) -> Option<&T> {
        self.element.front()
    }
    pub fn peek_back(&mut self) -> Option<&T> {
        self.element.back()
    }
    pub fn is_empty(&mut self) -> bool {
        self.element.is_empty()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_is_empty() {
        let mut q = super::Queue::new();
        q.enqueue(1);
        assert!(!q.is_empty());
        q.dequeue();
        assert!(q.is_empty())
    }
    #[test]
    fn test_enqueue() {
        let mut q = super::Queue::new();
        q.enqueue(1);
        assert!(!q.is_empty())
    }
    #[test]
    fn test_dequeue() {
        let mut q = super::Queue::new();
        q.enqueue(1);
        assert!(!q.is_empty());
        q.dequeue();
        assert!(q.is_empty())
    }
}
