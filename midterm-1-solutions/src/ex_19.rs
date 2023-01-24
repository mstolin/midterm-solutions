pub fn removeelfromvector(v: &mut Vec<usize>, n: usize) {
    for (i, val) in v.clone().iter().enumerate() {
        if *val == n {
            v.remove(i);
            continue;
        }
    }
}
