pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
    let mut acc = 0;
    let travel: Vec<i32> = std::iter::once(0)
        .chain(travel.into_iter().map(|t| {
            acc += t;
            acc
        }))
        .collect();
    let (p, m, g, last_p, last_m, last_g) = garbage.into_iter().enumerate().fold(
        (0, 0, 0, 0, 0, 0),
        |(mut p, mut m, mut g, mut last_p, mut last_m, mut last_g), (i, s)| {
            for &b in s.as_bytes() {
                match b {
                    b'P' => {
                        p += 1;
                        last_p = i;
                    }
                    b'M' => {
                        m += 1;
                        last_m = i;
                    }
                    _ => {
                        g += 1;
                        last_g = i;
                    }
                }
            }
            (p, m, g, last_p, last_m, last_g)
        },
    );
    p + m + g + travel[last_p] + travel[last_m] + travel[last_g]
}
#[cfg(test)]
mod tests {
    use crate::arrays::minimum_amount_of_time_to_collect_a_garbage::garbage_collection;

    #[test]
    fn test_garbage_collection() {
        assert_eq!(
            garbage_collection(
                vec![
                    String::from("G"),
                    String::from("P"),
                    String::from("GP"),
                    String::from("GG")
                ],
                vec![2, 4, 3]
            ),
            21
        );
    }
}
