/// LeetCode #2121 - Intervals Between Identical Elements
use std::collections::HashMap;

fn get_distances(arr: Vec<i32>) -> Vec<i64> {
    let mut groups: HashMap<i32, Vec<usize>> = HashMap::new();
    for (i, value) in arr.into_iter().enumerate() {
        groups.entry(value).or_default().push(i);
    }

    let mut ans = vec![0i64; groups.values().map(Vec::len).sum()];
    for indices in groups.values() {
        let total: i64 = indices.iter().map(|&i| i as i64).sum();
        let mut prefix = 0i64;

        for (rank, &idx) in indices.iter().enumerate() {
            let idx = idx as i64;
            let left = idx * rank as i64 - prefix;
            let right = (total - prefix - idx) - idx * (indices.len() - rank - 1) as i64;
            ans[idx as usize] = left + right;
            prefix += idx;
        }
    }

    ans
}

fn main() {
    println!("{:?}", get_distances(vec![2, 1, 3, 1, 2, 3, 3]));
}

#[cfg(test)]
mod tests {
    use super::get_distances;

    #[test]
    fn example_one() {
        assert_eq!(
            get_distances(vec![2, 1, 3, 1, 2, 3, 3]),
            vec![4, 2, 7, 2, 4, 4, 5]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(get_distances(vec![10, 5, 10, 10]), vec![5, 0, 3, 4]);
    }
}
