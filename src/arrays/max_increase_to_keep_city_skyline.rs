use std::cmp::min;

pub struct Solution {}
impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        for i in 0..grid.len() {
            let horizontal_max = grid[i].iter().max().unwrap();

            for j in 0..grid.len() {
                let mut vertical_max = 0;
                for k in 0..grid.len() {
                    if grid[k][j] > vertical_max {
                        vertical_max = grid[k][j];
                    }
                }

                let max_possible_height = min(horizontal_max, &vertical_max);
                if grid[i][j] < *max_possible_height {
                    result += max_possible_height - grid[i][j];
                }
            }
        }

        result
    }

    pub fn max_increase_keeping_skyline2(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut horizontal_maxs = vec![0; grid.len()];
        let mut vertical_maxs = vec![0; grid.len()];

        for i in 0..grid.len() {
            for j in 0..grid.len() {
                horizontal_maxs[i] = horizontal_maxs[i].max(grid[i][j]);
                vertical_maxs[j] = vertical_maxs[j].max(grid[i][j]);
            }
        }

        for i in 0..grid.len() {
            for j in 0..grid.len() {
                let max_possible_height = horizontal_maxs[i].min(vertical_maxs[j]);
                if grid[i][j] < max_possible_height {
                    result += max_possible_height - grid[i][j];
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::arrays::max_increase_to_keep_city_skyline::Solution;

    #[test]
    fn test_max_increase_keeping_skyline() {
        assert_eq!(
            Solution::max_increase_keeping_skyline(vec![
                vec![3, 0, 8, 4],
                vec![2, 4, 5, 7],
                vec![9, 2, 6, 3],
                vec![0, 3, 1, 0]
            ]),
            35
        )
    }

    #[test]
    fn test_max_increase_keeping_skyline2() {
        assert_eq!(
            Solution::max_increase_keeping_skyline(vec![
                vec![3, 0, 8, 4],
                vec![2, 4, 5, 7],
                vec![9, 2, 6, 3],
                vec![0, 3, 1, 0]
            ]),
            35
        )
    }
}
