pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
    points.sort_by_key(|point| point[0]);
    (0..points.len() - 1)
        .map(|i| points[i + 1][0] - points[i][0])
        .max()
        .unwrap() as i32
}

pub fn max_width_of_vertical_area2(mut points: Vec<Vec<i32>>) -> i32 {
    points.sort_by_key(|point| point[0]);
    points
        .windows(2)
        .map(|pair| pair[1][0] - pair[0][0])
        .max()
        .unwrap() as i32
}

#[cfg(test)]
mod tests {
    use crate::arrays::widest_vertical_area_between_two_points_containing_no_points::{
        max_width_of_vertical_area, max_width_of_vertical_area2,
    };

    #[test]
    fn test_max_width_of_vertical_area() {
        let points = vec![
            vec![1, 5],
            vec![1, 70],
            vec![3, 1000],
            vec![55, 700],
            vec![999876789, 53],
            vec![987853567, 12],
        ];

        assert_eq!(max_width_of_vertical_area(points), 987853512);

        let points = vec![vec![8, 7], vec![9, 9], vec![7, 4], vec![9, 7]];

        assert_eq!(max_width_of_vertical_area(points), 1);

        let points = vec![
            vec![3, 1],
            vec![9, 0],
            vec![1, 0],
            vec![1, 4],
            vec![5, 3],
            vec![8, 8],
        ];

        assert_eq!(max_width_of_vertical_area(points), 3);
    }

    #[test]
    fn test_max_width_of_vertical_area2() {
        let points = vec![
            vec![1, 5],
            vec![1, 70],
            vec![3, 1000],
            vec![55, 700],
            vec![999876789, 53],
            vec![987853567, 12],
        ];

        assert_eq!(max_width_of_vertical_area2(points), 987853512);

        let points = vec![vec![8, 7], vec![9, 9], vec![7, 4], vec![9, 7]];

        assert_eq!(max_width_of_vertical_area2(points), 1);

        let points = vec![
            vec![3, 1],
            vec![9, 0],
            vec![1, 0],
            vec![1, 4],
            vec![5, 3],
            vec![8, 8],
        ];

        assert_eq!(max_width_of_vertical_area2(points), 3);
    }
}
