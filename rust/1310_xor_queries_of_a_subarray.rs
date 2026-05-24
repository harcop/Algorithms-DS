/// LeetCode #1310 - XOR Queries of a Subarray
fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut pref = vec![0];
    for &x in &arr {
        pref.push(pref.last().unwrap() ^ x);
    }
    queries
        .into_iter()
        .map(|q| pref[q[1] as usize + 1] ^ pref[q[0] as usize])
        .collect()
}

fn main() {
    println!("{:?}", xor_queries(vec![1, 3, 4, 8], vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![2, 3]]));
}

#[cfg(test)]
mod tests {
    use super::xor_queries;

    #[test]
    fn example_one() {
        assert_eq!(xor_queries(vec![1, 3, 4, 8], vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![2, 3]]), vec![2, 7, 14, 12]);
    }

    #[test]
    fn example_two() {
        assert_eq!(xor_queries(vec![4, 8, 2, 10], vec![vec![2, 3], vec![1, 3], vec![0, 0], vec![0, 3]]), vec![8, 0, 4, 4]);
    }
}
