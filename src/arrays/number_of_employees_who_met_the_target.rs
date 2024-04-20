pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
    hours.iter().filter(|&&hour| hour >= target).count() as i32
}

#[cfg(test)]
mod tests {
    use crate::arrays::number_of_employees_who_met_the_target::number_of_employees_who_met_target;

    #[test]
    fn test_number_of_employees_who_met_target() {
        assert_eq!(
            number_of_employees_who_met_target(vec![0, 1, 2, 3, 4], 2),
            3
        );

        assert_eq!(
            number_of_employees_who_met_target(vec![5, 1, 4, 2, 2], 6),
            0
        );
    }
}
