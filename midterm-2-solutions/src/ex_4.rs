// ##########

use std::borrow::{Borrow, BorrowMut};
use std::cmp::Ordering;
use std::ops::Deref;

#[derive(Debug)]
pub struct List<T> {
    pub head: Link<T>,
    pub len: i32,
}

type Link<T> = Option<Box<Node<T>>>;
#[derive(Debug)]
pub struct Node<T> {
    pub elem: T,
    pub next: Link<T>,
}

#[derive(Debug)]
pub struct Content {
    s : String, b : bool, i : i32,
}
impl Content {
    pub fn new_with(s:String, b:bool, i:i32) -> Content {
        return Content{s,b,i};
    }
}

// ##########

impl PartialEq<Self> for Content {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i
    }
}

impl PartialOrd for Content {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }

    fn lt(&self, other: &Self) -> bool {
        self.i < other.i
    }

    fn le(&self, other: &Self) -> bool {
        self.i <= other.i
    }

    fn gt(&self, other: &Self) -> bool {
        self.i > other.i
    }

    fn ge(&self, other: &Self) -> bool {
        self.i >= other.i
    }
}

impl<T> Node<T> {
    pub fn new(elem: T, next: Link<T>) -> Self {
        Node {
            elem,
            next,
        }
    }
}

impl<T> Node<T>  where T: PartialOrd {
    fn add(&mut self, elem: T) {
        match self.next.as_mut() {
            None => self.next = Some(Box::new(Node::new(elem, None))),
            Some(next_node) => {
                if elem < next_node.elem {
                    // replace with new
                    let old = self.next.take();
                    self.next = Some(Box::new(Node::new(elem, old)));
                } else {
                    // try at next
                    next_node.add(elem);
                }
            }
        }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            len: 0,
        }
    }

    pub fn size(&self) -> i32 {
        self.len
    }
}

impl<T> List<T> where T: PartialOrd {
    pub fn add(&mut self, elem: T) {
        match self.head.as_mut() {
            None => self.head = Some(Box::new(Node::new(elem, None))),
            Some(head) => {
                if elem < head.elem {
                    // replace head
                    let old_head = self.head.take();
                    let new_node = Some(Box::new(Node::new(elem, old_head)));
                    self.head = new_node;
                } else {
                    head.add(elem);
                }
            }
        }
        self.len+=1;
    }
}

/*impl<T> List<T> where T: PartialEq {
    pub fn contains(&mut self, elem: T) -> bool {
        match self.head.take() {
            None => false,
            Some(n) => self._contains(n, &elem)
        }
    }

    fn _contains(&self, node: Box<Node<T>>, elem: &T) -> bool {
        if node.elem == *elem {
            true
        } else {
            match node.next {
                None => false,
                Some(next) => self._contains(next, elem)
            }
        }
    }
}*/