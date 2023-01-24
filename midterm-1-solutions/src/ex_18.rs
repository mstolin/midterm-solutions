pub fn veclengths(v: &Vec<String>) -> Vec<i32> {
    v.iter().map(|s| s.len() as i32).collect::<Vec<i32>>()
}
