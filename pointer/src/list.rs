use std::rc::Rc;

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

#[cfg(test)]
mod tests {
    use super::*;

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
