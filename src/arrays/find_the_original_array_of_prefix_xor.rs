pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
    let length = pref.len();
    let mut result: Vec<i32> = vec![0; pref.len()];

    if length == 0 {
        return result;
    }

    result[0] = pref[0];
    for i in 1..length {
        result[i] = pref[i - 1] ^ pref[i];
    }

    result
}

pub fn find_array2(mut pref: Vec<i32>) -> Vec<i32> {
    for i in (1..pref.len()).rev() {
        pref[i] ^= pref[i - 1];
    }

    pref
}

#[cfg(test)]
mod tests {
    use crate::arrays::find_the_original_array_of_prefix_xor::{find_array, find_array2};

    #[test]
    fn test_find_array() {
        assert_eq!(find_array(vec![5, 2, 0, 3, 1]), vec![5, 7, 2, 3, 2]);
        assert_eq!(find_array(vec![13]), [13]);
        assert_eq!(find_array(vec![]), []);
    }

    #[test]
    fn test_find_arra2() {
        assert_eq!(find_array2(vec![5, 2, 0, 3, 1]), vec![5, 7, 2, 3, 2]);
        assert_eq!(find_array2(vec![13]), [13]);
        assert_eq!(find_array2(vec![]), []);
    }
}
