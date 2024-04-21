pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let min_greatest_candies_number = *candies.iter().max().unwrap();
    let mut result: Vec<bool> = vec![];

    for candy_number in candies {
        let elem = candy_number + extra_candies >= min_greatest_candies_number;
        result.push(elem);
    }

    result
}

pub fn kids_with_candies2(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let min_greatest_candies_number = *candies.iter().max().unwrap();
    candies
        .into_iter()
        .map(|candy_number| candy_number + extra_candies >= min_greatest_candies_number)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::arrays::kids_with_the_greatest_number_of_candies::{
        kids_with_candies, kids_with_candies2,
    };

    #[test]
    fn test_kids_with_candies() {
        assert_eq!(
            kids_with_candies(vec![2, 3, 5, 1, 3], 3),
            vec![true, true, true, false, true]
        );

        assert_eq!(
            kids_with_candies(vec![4, 2, 1, 1, 2], 1),
            vec![true, false, false, false, false]
        );
    }

    #[test]
    fn test_kids_with_candies2() {
        assert_eq!(
            kids_with_candies2(vec![2, 3, 5, 1, 3], 3),
            vec![true, true, true, false, true]
        );

        assert_eq!(
            kids_with_candies2(vec![4, 2, 1, 1, 2], 1),
            vec![true, false, false, false, false]
        );
    }
}
