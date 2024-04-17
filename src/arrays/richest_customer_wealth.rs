pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut max_wealth: i32 = 0;

    for account in accounts {
        let current_wealth: i32 = account.iter().sum();
        if current_wealth > max_wealth {
            max_wealth = current_wealth;
        }
    }

    max_wealth
}

pub fn maximum_wealth2(accounts: Vec<Vec<i32>>) -> i32 {
    accounts
        .iter()
        .map(|account| account.iter().sum())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::arrays::richest_customer_wealth::{maximum_wealth, maximum_wealth2};

    #[test]
    fn test_maximum_wealth() {
        assert_eq!(maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]), 6);
        assert_eq!(maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]]), 10);
    }

    #[test]
    fn test_maximum_wealth2() {
        assert_eq!(maximum_wealth2(vec![vec![1, 2, 3], vec![3, 2, 1]]), 6);
        assert_eq!(
            maximum_wealth2(vec![vec![1, 5], vec![7, 3], vec![3, 5]]),
            10
        );
    }
}
