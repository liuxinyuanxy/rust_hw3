use std::cell::RefCell;
pub struct MyStack<T> {
    data: RefCell<Vec<T>>,
}

impl<T> MyStack<T> {
    pub fn new() -> MyStack<T> {
        MyStack {
            data: RefCell::new(Vec::new()),
        }
    }

    pub fn push(&self, item: T) {
        self.data.borrow_mut().push(item);
    }

    pub fn pop(&self) -> Option<T> {
        self.data.borrow_mut().pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_stack() {
        let stack = MyStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));

        stack.push(4);

        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }
}
