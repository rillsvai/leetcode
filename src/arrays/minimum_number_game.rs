pub fn number_game(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort();
    nums.chunks(2)
        .flat_map(|pair| [pair[1], pair[0]])
        .collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::number_game;

    #[test]
    fn test_number_game() {
        assert_eq!(number_game(vec![5, 4, 2, 3]), vec![3, 2, 5, 4]);
    }
}
