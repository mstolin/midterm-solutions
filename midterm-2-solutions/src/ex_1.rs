use core::fmt::{Debug, Display};

pub trait Nextable {
    fn gimme_next(&self) -> Option<Box<Self>>;
}

impl Nextable for i32 {
    fn gimme_next(&self) -> Option<Box<Self>> {
        Some(Box::new(self + 1))
    }
}

impl Nextable for char {
    fn gimme_next(&self) -> Option<Box<Self>> {
        let as_num = (*self as u32) + 1;
        let next = std::char::from_u32(as_num);
        next.map(|n| Box::new(n))
    }
}

pub fn printnext<T: Nextable + Debug>(n: T) {
    println!("next of {:?} is {:?}", n, n.gimme_next())
}