pub struct Solution {}

impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut point_counts = vec![0; queries.len()];

        for (i, circle_data) in queries.into_iter().enumerate() {
            let mut count = 0;

            for point in points.iter() {
                if (point[0] - circle_data[0]).pow(2) + (point[1] - circle_data[1]).pow(2)
                    <= circle_data[2].pow(2)
                {
                    count += 1;
                }
            }

            point_counts[i] = count;
        }
        return point_counts;
    }
}

#[cfg(test)]
mod tests {
    use crate::arrays::queries_on_number_of_points_inside_a_circle::Solution;

    #[test]
    fn test_count_points() {
        assert_eq!(
            Solution::count_points(
                vec![vec![1, 3], vec![3, 3], vec![5, 3], vec![2, 2]],
                vec![vec![2, 3, 1], vec![4, 3, 1], vec![1, 1, 2]]
            ),
            vec![3, 2, 2]
        );
    }
}
