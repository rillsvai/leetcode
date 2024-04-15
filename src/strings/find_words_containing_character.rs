pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
    let mut result = vec![];

    for i in 0..words.len() {
        if words[i].contains(x) {
            result.push(i as i32);
        }
    }

    result
}

pub fn find_words_containing2(words: Vec<String>, x: char) -> Vec<i32> {
    (0..words.len())
        .filter(|i| words[*i].contains(x))
        .map(|i| i as i32)
        .collect::<Vec<i32>>()
}

pub fn find_words_containing3(words: Vec<String>, x: char) -> Vec<i32> {
    (0..words.len())
        .filter_map(|i| words[i].contains(x).then(|| i as i32))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::strings::find_words_containing_character::{
        find_words_containing, find_words_containing2,
    };

    #[test]
    fn test_find_words_containing() {
        assert_eq!(
            find_words_containing(vec!["leet".into(), "code".into()], 'e'),
            [0, 1]
        );
    }

    #[test]
    fn test_find_words_containing2() {
        assert_eq!(
            find_words_containing2(vec!["leet".into(), "code".into()], 'e'),
            [0, 1]
        );
    }
}
