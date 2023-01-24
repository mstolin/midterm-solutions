#[derive(Debug)]
pub enum Z {
    Y1(i32, i32),
    Y2(bool, String),
}

pub fn maybelength(x: &Z) -> Option<usize> {
    match x {
        Z::Y1(_, _) => None,
        Z::Y2(_, s) => Some(s.len()),
    }
}
