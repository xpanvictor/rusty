use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
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

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

fn main() {
    let a = Rc::new(Cons(
        5,
        Rc::new(
            Cons(
            10,
            Rc::new(Nil)
            )
        )
    ));
    println!("Count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(
        3,
        Rc::clone(&a)
    );
    println!("Count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(
            4,
            Rc::clone(&a)
        );
        println!("Count after creating c = {}", Rc::strong_count(&a));
    }
    println!("Count after dropping c = {}", Rc::strong_count(&a));
    let val = 5;
    let x = MyBox::new(val);
    assert_eq!(5, val);
    assert_eq!(5, *x);

    let leaf = Rc::new(
        Node {
            value: 3,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        }
    );
    println!("leaf parent initially = {:?}", leaf.parent.borrow().upgrade());
    let branch = Rc::new(
        Node {
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()),
        }
    );
    // mutate leaf and make its parent branch
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("Hello, world!");
}
