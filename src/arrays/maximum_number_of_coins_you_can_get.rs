pub fn max_coins(mut piles: Vec<i32>) -> i32 {
    let n = piles.len() / 3;
    piles.sort_unstable();
    piles.into_iter().skip(n).step_by(2).sum()
}
