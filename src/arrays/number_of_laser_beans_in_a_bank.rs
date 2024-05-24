pub fn number_of_beams(bank: Vec<String>) -> i32 {
    let counts = bank
        .into_iter()
        .map(|row| row.bytes().filter(|byte| *byte == 49).count())
        .filter(|count| *count != 0);

    let mut prev = 0;
    let mut result = 0;

    for count in counts {
        result += count * prev;
        prev = count;
    }

    result as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_number_of_beams() {}
}
