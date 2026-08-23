use std::{cell::RefCell, rc::{Rc, Weak}};

#[derive(Debug)]
struct Node {
    value: u32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>
}

impl Node {
    fn show(&self) {
        println!("Printing elements");
        println!("########Parent######");
        if let Some(parent )=&self.parent.borrow().upgrade(){
            println!("The parent of this node is {}", parent.value)
        }else{
            println!("This node doesn't has a parent for now.")
        }
        println!("===> {}", &self.value);
        let childrens = self.children.borrow();
        for element in childrens.iter() {
            println!("===> {}", &element.value);
        }
    }

    fn new(value: u32) -> Rc<Node> {
        Rc::new(Node {
            value: value,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new())
        })
    }
}

fn main() {
    let leaf = Node::new(10);
    let branch = Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf), Node::new(15), Node::new(20), Node::new(25)]),
        parent: RefCell::new(Weak::new())
    };
    branch.show();
    leaf.show();
}
