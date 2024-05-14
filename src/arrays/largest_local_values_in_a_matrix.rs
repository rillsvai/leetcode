pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let result_length = grid.len() - 2;
    let mut result = vec![vec![0; result_length]; result_length];
    for n in 0..result_length {
        for k in 0..result_length {
            for i in n..3 + n {
                for j in k..3 + k {
                    result[n][k] = result[n][k].max(grid[i][j]);
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::arrays::largest_local_values_in_a_matrix::largest_local;

    #[test]
    fn test_largest_local() {
        assert_eq!(
            largest_local(vec![
                vec![9, 9, 8, 1],
                vec![5, 6, 2, 6],
                vec![8, 2, 6, 4],
                vec![6, 2, 2, 2]
            ]),
            vec![vec![9, 9], vec![8, 6]]
        );
    }
}
