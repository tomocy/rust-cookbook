use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {}

struct Node {
    value: i32,
    parent: Weak<Node>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: i32, parent: Weak<Node>, children: RefCell<Vec<Rc<Node>>>) -> Node {
        Node {
            value,
            parent,
            children,
        }
    }

    fn to_string_with_parent(&self) -> String {
        match self.parent.upgrade() {
            Some(parent) => format!("{} -> {}", parent.to_string_with_parent(), self.value),
            None => format!("{}", self.value),
        }
    }

    fn to_string_with_children(&self) -> String {
        let children = self
            .children
            .borrow()
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
        let branch = Rc::new(Node::new(5, Weak::new(), RefCell::new(Vec::new())));
        let leaf = Rc::new(Node::new(
            3,
            Rc::downgrade(&branch),
            RefCell::new(Vec::new()),
        ));
        branch.children.borrow_mut().push(Rc::clone(&leaf));

        assert_eq!("5 -> (3 -> ())", branch.to_string_with_children());
    }

    #[test]
    fn string_of_leaf_of_branch() {
        let branch = Rc::new(Node::new(5, Weak::new(), RefCell::new(Vec::new())));
        let leaf = Rc::new(Node::new(
            3,
            Rc::downgrade(&branch),
            RefCell::new(Vec::new()),
        ));
        branch.children.borrow_mut().push(Rc::clone(&leaf));

        assert_eq!("5 -> 3", leaf.to_string_with_parent());
    }
}
