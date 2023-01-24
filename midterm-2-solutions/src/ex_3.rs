pub fn basicbox_sum(v: Vec<String>) -> Vec<Box<usize>> {
    let mut r = Vec::new();
    let mut len: usize = 0;
    for s in v {
        let l = s.len();
        r.push(Box::new(l));
        len += l;
    }
    r.push(Box::new(len));
    r
}
