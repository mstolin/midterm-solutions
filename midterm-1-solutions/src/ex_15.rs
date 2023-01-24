use std::fmt::Formatter;
use std::mem;

#[derive(Debug)]
pub struct X {
    s: String,
    i: i32,
}

impl X {
    pub fn new() -> Self {
        X {
            s: "xxx".to_string(),
            i: 10,
        }
    }

    pub fn getstring(&mut self) -> String {
        mem::replace(&mut self.s, "".to_string())
    }
}

impl std::fmt::Display for X {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        return write!(f, "S {}, I {}", self.s, self.i);
    }
}

#[derive(Debug)]
pub struct Y {
    z: bool,
    c: String,
}

impl Y {
    pub fn new() -> Self {
        Self {
            z: true,
            c: "op".to_string(),
        }
    }
    pub fn getstring(&mut self) -> String {
        mem::replace(&mut self.c, "".to_string())
    }
}

impl std::fmt::Display for Y {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        return write!(f, "B {}, C {}", self.z, self.c);
    }
}

pub fn swapstr(mut x: X, mut y: Y) -> (X, Y) {
    let old_s = x.s.clone();
    let old_c = y.c.clone();
    x.s = old_c;
    y.c = old_s;
    (x, y)
}
