/// LeetCode #1177 - Can Make Palindrome from Substring
fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let b = s.as_bytes();
    let n = b.len();
    let mut pref = vec![0i32; n + 1];
    for i in 0..n {
        pref[i + 1] = pref[i] ^ (1 << (b[i] - b'a'));
    }
    queries
        .into_iter()
        .map(|q| {
            let l = q[0] as usize;
            let r = q[1] as usize;
            let k = q[2];
            let mask = pref[r + 1] ^ pref[l];
            (mask.count_ones() as i32 / 2) <= k
        })
        .collect()
}

fn main() {
    let s = "abcda".to_string();
    let q = vec![vec![3, 3, 0], vec![1, 2, 0], vec![0, 3, 1]];
    println!("{:?}", can_make_pali_queries(s, q));
}

#[cfg(test)]
mod tests {
    use super::can_make_pali_queries;

    #[test]
    fn example_one() {
        assert_eq!(
            can_make_pali_queries(
                "abcda".to_string(),
                vec![vec![3, 3, 0], vec![1, 2, 0], vec![0, 4, 1]]
            ),
            vec![true, false, true]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            can_make_pali_queries("yzt".to_string(), vec![vec![0, 1, 1]]),
            vec![true]
        );
    }
}
