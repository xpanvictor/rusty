use std::ops::Deref;
use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let list = Cons(
        1,
        Box::new(Cons(
            2,
            Box::new(Cons(
                3,
                Box::new(Nil)
            ))
        ))
    );
    let val = 5;
    let a = MyBox::new(val);
    assert_eq!(5, val);
    assert_eq!(5, *a);
    println!("Hello, world!");
}
