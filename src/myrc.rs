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
        let boxed = Box::new(RefCell::new(Inner { data, count: 1 }));
        MyRc {
            ptr: Box::leak(boxed),
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
        let mut inner = self.ptr.borrow_mut();
        inner.count -= 1;
        if inner.count == 0 {
            std::mem::drop(inner);
            println!("dropped")
        }
    }
}

impl<'a, T> std::ops::Deref for MyRc<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.ptr.as_ptr().as_ref().unwrap().data }
    }
}

// test

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_myrc() {
        let a: MyRc<i32> = MyRc::new(1);
        assert_eq!(*a, 1);
        assert_eq!(a.strong_count(), 1);
        let b = a.clone();
        assert_eq!(*b, 1);
        assert_eq!(a.strong_count(), 2);
        assert_eq!(b.strong_count(), 2);

        {
            let c = b.clone();
            assert_eq!(*c, 1);
            assert_eq!(a.strong_count(), 3);
            assert_eq!(b.strong_count(), 3);
            assert_eq!(c.strong_count(), 3);
        }

        assert_eq!(a.strong_count(), 2);
        assert_eq!(b.strong_count(), 2);

        std::mem::drop(a);

        assert_eq!(*b, 1);
        assert_eq!(b.strong_count(), 1);

        std::mem::drop(b);

        println!("test_myrc finished")
    }
}
