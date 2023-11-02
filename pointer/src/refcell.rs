use crate::refcell::List::{Cons, Nil};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq)]
pub enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    pub fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
        assert_eq!(1, Rc::strong_count(&a));
        assert_eq!(Some(&RefCell::new(Rc::new(Nil))), a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
        assert_eq!(2, Rc::strong_count(&a));
        assert_eq!(1, Rc::strong_count(&b));
        assert_eq!(
            Some(&RefCell::new(Rc::new(Cons(5, RefCell::new(Rc::new(Nil)))))),
            b.tail()
        );
    }
}
