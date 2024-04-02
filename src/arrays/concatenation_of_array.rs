
pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let mut result = nums.clone();
    result.extend(nums);
    result
}

pub fn get_concatenation2(nums: Vec<i32>) -> Vec<i32> {
    let length: usize = nums.len();
    let mut result: Vec<i32> = vec![0; length * 2];

    for (i, num) in nums.iter().enumerate() {
        result[i] = *num;
        result[i+length] = *num;
    }
    
    result

}


#[cfg(test)]
mod tests {
    use crate::arrays::concatenation_of_array::{get_concatenation, get_concatenation2};

    #[test]
    fn test_get_concatenation() {
        assert_eq!( get_concatenation(vec![1,2,3]), vec![1,2,3,1,2,3]);
        assert_eq!( get_concatenation(vec![]), vec![]);
    }

    #[test]
    fn test_get_concatenation2() {
        assert_eq!( get_concatenation2(vec![1,2,3]), vec![1,2,3,1,2,3]);
        assert_eq!( get_concatenation2(vec![]), vec![]);
    }
}