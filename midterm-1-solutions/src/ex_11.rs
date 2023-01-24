pub mod enumx {
    pub enum X {
        Y(String),
    }
}

pub mod structx {
    pub struct X {
        pub i: String,
    }
}

pub mod modfun {
    use crate::enumx::X as EX;
    use crate::structx::X as SX;

    pub fn longer(x1: &EX, x2: &SX) -> i32 {
        if let crate::enumx::X::Y(n) = x1 {
            let l1 = n.len() as i32;
            let l2 = x2.i.len() as i32;
            return if l1 > l2 { l1 } else { l2 };
        }
        return 0;
    }
}
