#[derive(PartialEq, Debug)]
pub struct SubrectangleQueries {
    matrix: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SubrectangleQueries {
    pub fn new(rectangle: Vec<Vec<i32>>) -> Self {
        SubrectangleQueries { matrix: rectangle }
    }

    pub fn update_subrectangle(
        &mut self,
        row1: i32,
        col1: i32,
        row2: i32,
        col2: i32,
        new_value: i32,
    ) {
        for i in row1..=row2 {
            for j in col1..=col2 {
                self.matrix[i as usize][j as usize] = new_value;
            }
        }
    }

    pub fn get_value(&self, row: i32, col: i32) -> i32 {
        self.matrix[row as usize][col as usize]
    }
}

/**
 * Your SubrectangleQueries object will be instantiated and called as such:
 * let obj = SubrectangleQueries::new(rectangle);
 * obj.update_subrectangle(row1, col1, row2, col2, newValue);
 * let ret_2: i32 = obj.get_value(row, col);
 */

#[cfg(test)]
mod tests {
    use crate::arrays::subrectangle_queries::SubrectangleQueries;

    #[test]
    fn test_get_value() {
        let matrix = SubrectangleQueries {
            matrix: vec![vec![1, 2, 1], vec![4, 3, 4], vec![3, 2, 1], vec![1, 1, 1]],
        };

        assert_eq!(matrix.get_value(1, 0), 4);
        assert_eq!(matrix.get_value(2, 1), 2);
    }

    #[test]
    fn test_update_subrectangle() {
        let mut matrix = SubrectangleQueries {
            matrix: vec![vec![1, 2, 1], vec![4, 3, 4], vec![3, 2, 1], vec![1, 1, 1]],
        };

        matrix.update_subrectangle(0, 0, 3, 2, 5);

        assert_eq!(
            matrix,
            SubrectangleQueries {
                matrix: vec![vec![5, 5, 5], vec![5, 5, 5], vec![5, 5, 5], vec![5, 5, 5]],
            }
        );

        matrix.update_subrectangle(3, 1, 3, 2, 7);

        assert_eq!(
            matrix,
            SubrectangleQueries {
                matrix: vec![vec![5, 5, 5], vec![5, 5, 5], vec![5, 5, 5], vec![5, 7, 7]],
            }
        );

        matrix.update_subrectangle(2, 2, 2, 2, 8);

        assert_eq!(
            matrix,
            SubrectangleQueries {
                matrix: vec![vec![5, 5, 5], vec![5, 5, 5], vec![5, 5, 8], vec![5, 7, 7]],
            }
        );
    }
}
