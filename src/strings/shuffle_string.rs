pub fn restore_string(s: String, indices: Vec<i32>) -> String {
    let mut res = vec![' '; indices.len()];
    for (i, &x) in indices.iter().enumerate() {
        res[x as usize] = s.as_bytes()[i] as char;
    }
    res.iter().collect::<String>()
}
