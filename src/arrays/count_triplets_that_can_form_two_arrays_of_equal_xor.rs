pub fn count_triplets(arr: Vec<i32>) -> i32 {
    let mut ans = 0 as i32;
    let n = arr.len();
    for i in 0..n {
        let mut val = arr[i];
        for k in i + 1..n {
            val ^= arr[k];
            if val == 0 {
                ans += (k - i) as i32;
            }
        }
    }
    ans
}
