#[derive(Debug)]
pub enum A {
    A1(i32, i32, i32),
    A2(char, char),
}

#[derive(Debug)]
pub enum B {
    B1(i32, i32),
    B2(String),
}

pub fn bfroma(a: A) -> B {
    match a {
        A::A1(a, b, c) => B::B2(format!("{}-{}-{}", a, b, c)),
        A::A2(x, y) => B::B1(x as i32, y as i32),
    }
}
