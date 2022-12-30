use core::fmt::Debug;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::{Formatter};
use std::rc::Rc;

// ##########

pub trait SameBool{
    fn samebool(&self, other:&Self)->bool;
}
#[derive(Debug)]
pub struct Content{
    pub i:i32,
    pub b:bool
}
impl Content {
    pub fn new_with(i: i32, b: bool) -> Content {
        Content { i, b }
    }
}
type NodeRef<T> = Rc<RefCell<Node<T>>>;
struct Node<T> {
    inner_value: T,
    adjacent: Vec<NodeRef<T>>,
}
impl<T:Debug> Debug for Node<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"iv: {:?}, adj: {}", self.inner_value, self.adjacent.len())
    }
}
#[derive(Debug)]
pub struct Graph<T> {
    nodes: Vec<NodeRef<T>>,
}

// ##########

impl PartialEq for Content {
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

impl SameBool for Content {
    fn samebool(&self, other: &Self) -> bool {
        self.b == other.b
    }
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph { nodes: Vec::new() }
    }
}