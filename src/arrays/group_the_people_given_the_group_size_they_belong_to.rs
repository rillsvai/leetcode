use std::collections::HashMap;

pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    let mut groups = HashMap::<i32, Vec<i32>>::new();

    for (id, &group_size) in group_sizes.iter().enumerate() {
        let group = groups.entry(group_size).or_insert(vec![]);

        if group.len() == group_size as usize {
            result.push(group.clone());
            *group = vec![];
        }

        group.push(id as i32);
    }

    for group in groups {
        result.push(group.1);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::arrays::group_the_people_given_the_group_size_they_belong_to::group_the_people;

    #[test]
    #[ignore = "inconsistent order from hash map"]
    fn test_group_the_people() {
        assert_eq!(
            group_the_people(vec![3, 3, 3, 3, 3, 1, 3]),
            vec![vec![0, 1, 2], vec![3, 4, 6], vec![5]]
        )
    }
}
