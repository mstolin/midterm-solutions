fn replace(v: &mut Vec<String>, i: usize) {
    match v.get(i) {
        None => (),
        Some(val) => match v.get(i - 1) {
            None => (),
            Some(prev) => {
                v[i] = format!("{} {}", prev, val);
            }
        },
    }
}

pub fn swapelconcat(v: &mut Vec<String>, i: usize, j: usize) -> Option<&Vec<String>> {
    for index in [i, j] {
        replace(v, index);
    }
    Some(v)
}
