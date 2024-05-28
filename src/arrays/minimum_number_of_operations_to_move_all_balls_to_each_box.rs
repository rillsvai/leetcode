pub fn min_operations(boxes: String) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0; boxes.len()];

    for i in 0..boxes.len() {
        result[i] = boxes
            .bytes()
            .enumerate()
            .map(|(j, byte)| ((byte - 48) as usize * j.abs_diff(i)) as i32)
            .sum()
    }

    result
}

pub fn min_operations2(boxes: String) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0; boxes.len()];

    let mut operations_count = 0;
    let mut non_empty_boxes_count = 0;
    for (i, byte) in boxes.bytes().enumerate() {
        result[i] += operations_count as i32;

        non_empty_boxes_count += byte - 48;

        operations_count += non_empty_boxes_count;
    }

    operations_count = 0;
    non_empty_boxes_count = 0;
    for (i, byte) in boxes.bytes().enumerate().rev() {
        result[i] += operations_count as i32;

        non_empty_boxes_count += byte - 48;

        operations_count += non_empty_boxes_count;
    }

    result
}
#[cfg(test)]
mod tests {
    use crate::arrays::minimum_number_of_operations_to_move_all_balls_to_each_box::{
        min_operations, min_operations2,
    };

    #[test]
    fn test_min_operations() {
        assert_eq!(min_operations(String::from("110")), vec![1, 1, 3]);
        assert_eq!(
            min_operations(String::from("001011")),
            vec![11, 8, 5, 4, 3, 4]
        );
    }

    #[test]
    fn test_min_operations2() {
        assert_eq!(min_operations2(String::from("110")), vec![1, 1, 3]);
        assert_eq!(
            min_operations2(String::from("001011")),
            vec![11, 8, 5, 4, 3, 4]
        );
    }
}
