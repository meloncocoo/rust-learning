use std::{ops::Deref, rc::Rc};
pub mod message;
pub mod tree;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// Shows how to use a box to store an `i32` value on the heap
pub fn demo() {
    let b = Box::new(5);
    println!("b = {}", b);
}

/// The first attempt at defining an enum to represent a cons list data structure of `i32` values
#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

/// `Rc<T>` demo
#[derive(Debug)]
pub enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

/// Using the `List` type to store the list `1`, `2`, `3` would look like the code in Listing
pub fn cons_list() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    println!("list = {:?}", list);
}

pub struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

/// MyBox<T>
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_box() {
        let b = Box::new(5);
        assert_eq!(*b, 5);

        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn test_pointer() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn test_my_box() {
        let x = 5;
        let y = MyBox::new(5);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn test_drop_custom_smart_pointer() {
        let _a = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let _b = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
    }

    #[test]
    fn test_cons_list() {
        let a = Rc::new(RcList::Cons(
            5,
            Rc::new(RcList::Cons(10, Rc::new(RcList::Nil))),
        ));
        assert_eq!(1, Rc::strong_count(&a));
        let _b = RcList::Cons(3, Rc::clone(&a));
        assert_eq!(2, Rc::strong_count(&a));
        let _c = RcList::Cons(4, Rc::clone(&a));
        assert_eq!(3, Rc::strong_count(&a));
    }
}
