pub fn most_words_found(sentences: Vec<String>) -> i32 {
    sentences
        .iter()
        .map(|sentence| sentence.split(' ').count())
        .max()
        .unwrap() as i32
}

#[cfg(test)]
mod tests {
    use crate::arrays::maximum_number_of_words_found_in_sentences::most_words_found;

    #[test]
    fn test_most_words_found() {
        assert_eq!(
            most_words_found(vec![
                "alice and bob love leetcode".into(),
                "i think so too".into(),
                "this is great thanks very much".into()
            ]),
            6
        )
    }
}
