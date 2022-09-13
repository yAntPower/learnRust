use crate::List::{Cons, Nil};
use crate::List2::{Cons as Cons2, Nil as Nil2};
use std::cell::RefCell;
use std::ops::Deref;
mod refcell;
use std::rc::Rc;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
struct CustomSmartPointer {
    date: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("{}", self.date);
    }
}
fn hello(s: &str) {
    print!("{}", s);
}
fn main() {
    let s = MyBox::new(String::from("test Box<T>"));
    hello(&s);
    let cc = CustomSmartPointer {
        date: String::from("CustomSmartPointer"),
    };
    // let c = Cons(4,Box::new(aa)); //aa变量已经被移动到了变量b，所以这次无法将aa变量赋给c 解决方案就是采用List2中的Rc<T>
    let d = 3;
    let e = MyBox::new(d);
    assert_eq!(3, d);
    assert_eq!(3, *e);
    let f = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    println!("计数1:{}", Rc::strong_count(&f));
    let g = Cons2(4, Rc::clone(&f));
    println!("计数2:{}", Rc::strong_count(&f));
    let i = Cons2(3, Rc::clone(&f));
    println!("计数3:{}", Rc::strong_count(&f));
    {
        let j = Cons2(3, Rc::clone(&f));
        println!("计数4:{}", Rc::strong_count(&f));
    }
    println!("计数5:{}", Rc::strong_count(&f));
}
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

enum List2 {
    Cons(i32, Rc<List2>),
    Nil,
}

fn aaaaa() {
   let value = Rc::new(RefCell::new(5));

   let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

   let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
   let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

   *value.borrow_mut() += 10;
}