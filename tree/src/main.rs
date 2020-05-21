use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {}

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: Vec<Rc<Node>>,
}

impl Node {
    fn new(value: i32, parent: RefCell<Weak<Node>>, children: Vec<Rc<Node>>) -> Node {
        Node {
            value,
            parent,
            children,
        }
    }

    fn to_string_with_parent(&self) -> String {
        match self.parent.borrow().upgrade() {
            Some(parent) => format!("{} -> {}", parent.to_string_with_parent(), self.value),
            None => format!("{}", self.value),
        }
    }

    fn to_string_with_children(&self) -> String {
        let children = self
            .children
            .iter()
            .map(|child| child.to_string_with_children())
            .collect::<Vec<String>>()
            .join(", ");

        format!("{} -> ({})", self.value, children)
    }
}

mod tests {
    use super::*;

    #[test]
    fn string_of_branch_with_leaf() {
        let leaf = Rc::new(Node::new(3, RefCell::new(Weak::new()), Vec::new()));
        let branch = Rc::new(Node::new(
            5,
            RefCell::new(Weak::new()),
            vec![Rc::clone(&leaf)],
        ));
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        assert_eq!("5 -> (3 -> ())", branch.to_string_with_children());
    }

    #[test]
    fn string_of_leaf_of_branch() {
        let leaf = Rc::new(Node::new(3, RefCell::new(Weak::new()), Vec::new()));
        let branch = Rc::new(Node::new(
            5,
            RefCell::new(Weak::new()),
            vec![Rc::clone(&leaf)],
        ));
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        assert_eq!("5 -> 3", leaf.to_string_with_parent());
    }
}
