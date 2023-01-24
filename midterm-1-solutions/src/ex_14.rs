pub fn prevchar(c: char) -> char {
    let prev = (c as u32) - 1;
    std::char::from_u32(prev).expect("not able to get previous char")
}

pub fn replwithprev(s: &mut String) -> Result<String, ()> {
    if s.contains('a') {
        return Err(());
    }
    let res = s.chars().map(|c| prevchar(c)).collect::<String>();

    Ok(res)
}
