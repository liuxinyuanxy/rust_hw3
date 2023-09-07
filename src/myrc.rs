use std::cell::RefCell;

struct Inner<T> {
    data: T,
    count: usize,
}

pub struct MyRc<'a, T> {
    ptr: &'a RefCell<Inner<T>>,
}

impl<'a, T> MyRc<'a, T> {
    pub fn new(data: T) -> Self {
        MyRc {
            ptr: &RefCell::new(Inner { data, count: 1 }),
        }
    }

    pub fn clone(&self) -> Self {
        self.ptr.borrow_mut().count += 1;
        MyRc { ptr: self.ptr }
    }

    pub fn strong_count(&self) -> usize {
        self.ptr.borrow().count
    }
}

impl<'a, T> Drop for MyRc<'a, T> {
    fn drop(&mut self) {
        self.ptr.borrow_mut().count -= 1;
        if self.ptr.borrow_mut().count == 0 {
            println!("Dropping MyRc!");
            std::mem::drop(self.ptr);
        }
    }
}

impl<'a, T> std::ops::Deref for MyRc<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.ptr.borrow().data
    }
}

// test

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_myrc() {
        let a: MyRc<i32> = MyRc::new(1);
        assert_eq!(a.strong_count(), 1);
        let b: MyRc<i32> = a.clone();
        assert_eq!(a.strong_count(), 2);
        assert_eq!(b.strong_count(), 2);
        {
            let c: MyRc<i32> = a.clone();
            assert_eq!(a.strong_count(), 3);
            assert_eq!(b.strong_count(), 3);
            assert_eq!(c.strong_count(), 3);
        }
        assert_eq!(a.strong_count(), 2);
    }
}
