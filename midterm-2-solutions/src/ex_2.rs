pub struct Wrapper {
    pub v: Vec<i32>,
}

impl Wrapper {
    pub fn iter(&self) -> Iter {
        Iter::new(&self.v)
    }
}

pub struct Iter<'a> {
    v: Vec<&'a i32>,
    index: usize,
}

impl<'a> Iter<'a> {
    pub fn new(v: &'a Vec<i32>) -> Self {
        let odds: Vec<&i32> = v.iter().clone().filter(|n| *n % 2 == 1).collect();
        Self { v: odds , index: 0 }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.v.get(self.index) {
            None => None,
            Some(n) => {
                self.index += 1;
                Some(n)
            }
        }
    }
}