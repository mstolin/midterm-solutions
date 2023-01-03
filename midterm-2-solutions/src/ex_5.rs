use core::fmt::Debug;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::{Formatter};
use std::ops::Deref;
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

impl<T> Graph<T> where T: PartialOrd + SameBool {
    fn get_neighbours(&self, val: &T) -> Vec<NodeRef<T>> {
        self.nodes
            .iter()
            .filter(|n| n.as_ref().borrow().inner_value < *val)
            .map(|n| Rc::clone(n))
            .collect()
    }

    fn add_as_neighbour(&mut self, node: &NodeRef<T>) {
        self.nodes
            .iter()
            .filter(|n| n.as_ref().borrow().inner_value.samebool(&node.as_ref().borrow().inner_value))
            .for_each(|n| n.as_ref().borrow_mut().adjacent.push(Rc::clone(node)));
    }

    pub fn add_node(&mut self, inner_value: T) {
        // 1. get the neighbours
        let adjacent = self.get_neighbours(&inner_value);
        // 2. create the node
        let node = Node { inner_value, adjacent };
        let node: NodeRef<T> = Rc::new(RefCell::new(node));
        // 3. add the node as neighbour to others
        self.add_as_neighbour(&node);
        // 4. add node to graph
        self.nodes.push(node);
    }
}
