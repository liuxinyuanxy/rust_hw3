run `cargo test` to test myrc and mystack.

The code of macro is in main.rs

### About MyRc

写了一整天，记录一下想法

Rc的基础想法就是，在堆上分配一块内存，储存数据和引用计数，用不同的引用指向这同一快内存。

- clone 需要将计数加一，并复制一份引用。要复制一份引用，必须要求该引用是不可变引用，为了能够修改计数，使用 RefCell 的内部可变性。

- drop 需要将计数减一，并在计数清零时回收内存。 
- deref 返回内部值的引用。

因此最初的想法为，初始化时使用 Box 在堆上分配数据，clone 时复制一份不可变引用。即如下代码：

```rust
enum InnerBox<'a, T> {
    TrueBox(Box<RefCell<Inner<T>>>),
    FakeBox(&'a RefCell<Inner<T>>),
}

pub struct MyRc<'a, T> {
    ptr: InnerBox<'a, T>,
}

impl<'a, T> MyRc<'a, T> {
    pub fn new(data: T) -> Self {
        MyRc {
            ptr: InnerBox::TrueBox(Box::new(RefCell::new(Inner { data, count: 1 }))),
        }
    }

    pub fn clone(&self) -> Self {
        match &self.ptr {
            InnerBox::TrueBox(boxed) => {
                boxed.borrow_mut().count += 1;
                return MyRc {
                    ptr: InnerBox::FakeBox(boxed.as_ref()),
                };
            }
            InnerBox::FakeBox(rc) => {
                rc.borrow_mut().count += 1;
                return MyRc {
                    ptr: InnerBox::FakeBox(rc),
                };
            }
        }
    }
}
```

但是这个想法存在很大的问题，由于 Inner 的所有权在最初的 Box 中，最初的 box 结束生命周期后数据就会被释放，查询资料后考虑用 leak 获得一个引用，代码变成了：

```rust
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
}
```

根据文档，这样做的话，drop 时就需要使用`Box::from_raw`来释放内存以避免内存泄漏。

