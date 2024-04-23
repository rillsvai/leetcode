use std::collections::HashMap;

pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut occurences: HashMap<i32, usize> = HashMap::new();
    let mut result: Vec<Vec<i32>> = vec![];

    for num in nums {
        let entry = occurences.entry(num).or_insert(0);
        if result.len() < *entry + 1 {
            result.push(vec![]);
        }

        result[*entry].push(num);
        *entry += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::find_matrix;

    #[test]
    fn test_find_matrix() {
        assert_eq!(
            find_matrix(vec![1, 3, 4, 1, 2, 3, 1]),
            vec![vec![1, 3, 4, 2], vec![1, 3], vec![1]]
        );
    }
}

//   public List<List<Integer>> findMatrix(int[] A) {
//         ArrayList<List<Integer>> res = new ArrayList<>();
//         int n = A.length, count[] = new int[n + 1];
//         for (int a : A) {
//             if (res.size() < ++count[a])
//                 res.add(new ArrayList<>());
//             res.get(count[a]).add(a);
//         }
//         return res;
//     }
