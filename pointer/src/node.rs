use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub parent: RefCell<Weak<Node>>,
    pub children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    pub fn new(value: i32) -> Node {
        Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        }
    }

    // pub fn add_children(self, node: &Rc<Node>) {
    //     let branch = Rc::new(self);
    //     branch.children.borrow_mut().push(node.clone());
    //     *node.parent.borrow_mut() = Rc::downgrade(&branch);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let leaf = Rc::new(Node::new(3));
        let branch = Rc::new(Node::new(5));
        branch.children.borrow_mut().push(leaf.clone());
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(
            "Node: {}, {:?}, {:?}",
            branch.value, branch.parent, branch.children
        );
        // assert_eq!(
        //     branch,
        //     Rc::new(Node {
        //         value: 5,
        //         parent: RefCell::new(Weak::new()),
        //         children: RefCell::new(vec![Rc::new(Node {
        //             value: 3,
        //             parent: RefCell::new(Weak::new()),
        //             children: RefCell::new(vec![])
        //         })])
        //     })
        // );
    }
}
