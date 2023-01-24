use std::cmp::max;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub enum C {
    C1(i32, i32),
    C2(bool, String),
}

#[derive(Debug)]
pub struct D {
    c: C,
    s: String,
}
impl std::fmt::Display for D {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        return write!(f, "D: {} with {:?}", self.s, self.c);
    }
}

impl D {
    pub fn new() -> Self {
        Self {
            c: C::C1(0, 0),
            s: String::new(),
        }
    }

    pub fn new_with_C(c: C) -> Self {
        match c.clone() {
            C::C1(_, _) => Self {
                c,
                s: "not there".to_string(),
            },
            C::C2(_, s) => Self { c, s },
        }
    }

    pub fn larger(&self) -> i32 {
        match self.c.clone() {
            C::C1(_, _) => self.s.len() as i32,
            C::C2(_, s) => max(self.s.len() as i32, s.len() as i32),
        }
    }
}
