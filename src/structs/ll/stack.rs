use std::collections::LinkedList;

#[derive(Debug)]
pub struct Stack<T> {
    elements: LinkedList<T>,
}
impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            elements: LinkedList::new(),
        }
    }
    pub fn push(&mut self, value: T) {
        self.elements.push_front(value)
    }
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop_front()
    }
    pub fn peek(&mut self) -> Option<&T> {
        self.elements.front()
    }
    pub fn is_empty(&mut self) -> bool {
        self.elements.is_empty()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn stack_is_empty() {
        let mut s = super::Stack::new();
        s.push(10);
        assert!(!s.is_empty());
        s.pop();
        assert!(s.is_empty())
    }
    #[test]
    fn stack_push() {
        let mut s = super::Stack::new();
        s.push(10);
        assert!(!s.is_empty())
    }
    #[test]
    fn stack_pop() {
        let mut s = super::Stack::new();
        s.push(10);
        assert!(!s.is_empty());
        s.pop();
        assert!(s.is_empty())
    }
    #[test]
    fn stack_peek() {
        let mut s = super::Stack::new();
        s.push(10);
        assert!(!s.is_empty());
        assert!(s.peek() == Some(&10));
        s.pop();
        assert!(s.is_empty());
        assert!(s.peek() == None);
    }
}
