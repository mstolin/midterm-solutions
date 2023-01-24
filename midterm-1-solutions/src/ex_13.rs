pub struct Balance {
    pub amt: i32,
    pub active: bool,
}

impl Balance {
    pub fn maybericher(&self, ab: Balance) -> Option<bool> {
        if !self.active || !ab.active {
            return None;
        }
        let is_richer = self.amt > ab.amt;
        Option::from(is_richer)
    }
}
