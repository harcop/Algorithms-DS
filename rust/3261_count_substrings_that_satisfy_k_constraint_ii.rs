/// LeetCode #3261 - Count Substrings That Satisfy K-Constraint II
fn count_k_constraint_substrings(s: String, k: i32, queries: Vec<Vec<i32>>) -> Vec<i64> {
    let s = s.as_bytes();
    let n = s.len();
    let mut cnt = [0i32; 2];
    let mut i = 0;
    let mut d = vec![n; n];
    let mut pre = vec![0i64; n + 1];
    for j in 0..n {
        cnt[(s[j] - b'0') as usize] += 1;
        while cnt[0] > k && cnt[1] > k {
            d[i] = j;
            cnt[(s[i] - b'0') as usize] -= 1;
            i += 1;
        }
        pre[j + 1] = pre[j] + (j - i + 1) as i64;
    }
    let mut ans = Vec::with_capacity(queries.len());
    for q in queries {
        let l = q[0] as usize;
        let r = q[1] as usize;
        let p = (r + 1).min(d[l]);
        let a = (1 + p - l) as i64 * (p - l) as i64 / 2;
        let b = pre[r + 1] - pre[p];
        ans.push(a + b);
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        count_k_constraint_substrings("0001111".into(), 2, vec![vec![0, 6]])
    );
}

#[cfg(test)]
mod tests {
    use super::count_k_constraint_substrings;

    #[test]
    fn example1() {
        assert_eq!(
            count_k_constraint_substrings("0001111".into(), 2, vec![vec![0, 6]]),
            vec![26]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            count_k_constraint_substrings(
                "010101".into(),
                1,
                vec![vec![0, 5], vec![1, 4], vec![2, 3]]
            ),
            vec![15, 9, 3]
        );
    }
}
