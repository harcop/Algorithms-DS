/// LeetCode #2983 - Palindrome Rearrangement Queries
fn can_make_palindrome_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let n = s.len();
    let m = n / 2;
    let bytes = s.as_bytes();
    let left = &bytes[..m];
    let right: Vec<u8> = bytes[m..].iter().copied().rev().collect();

    let mut pre1 = vec![[0i32; 26]; m + 1];
    let mut pre2 = vec![[0i32; 26]; m + 1];
    let mut diff = vec![0i32; m + 1];
    for i in 0..m {
        pre1[i + 1] = pre1[i];
        pre2[i + 1] = pre2[i];
        pre1[i + 1][(left[i] - b'a') as usize] += 1;
        pre2[i + 1][(right[i] - b'a') as usize] += 1;
        diff[i + 1] = diff[i] + if left[i] != right[i] { 1 } else { 0 };
    }

    let count = |pre: &[[i32; 26]], i: usize, j: usize| -> [i32; 26] {
        let mut res = [0; 26];
        for c in 0..26 {
            res[c] = pre[j + 1][c] - pre[i][c];
        }
        res
    };

    let sub = |cnt1: [i32; 26], cnt2: [i32; 26]| -> Option<[i32; 26]> {
        let mut res = [0; 26];
        for c in 0..26 {
            if cnt1[c] - cnt2[c] < 0 {
                return None;
            }
            res[c] = cnt1[c] - cnt2[c];
        }
        Some(res)
    };

    let check = |pre_a: &[[i32; 26]],
                 pre_b: &[[i32; 26]],
                 a: usize,
                 b: usize,
                 c: usize,
                 d: usize,
                 diff: &[i32],
                 m: usize|
     -> bool {
        if diff[a] > 0 || diff[m] - diff[b.max(d) + 1] > 0 {
            return false;
        }
        if d <= b {
            return count(pre_a, a, b) == count(pre_b, a, b);
        }
        if b < c {
            return diff[c] - diff[b + 1] == 0
                && count(pre_a, a, b) == count(pre_b, a, b)
                && count(pre_a, c, d) == count(pre_b, c, d);
        }
        match (
            sub(count(pre_a, a, b), count(pre_b, a, c - 1)),
            sub(count(pre_b, c, d), count(pre_a, b + 1, d)),
        ) {
            (Some(c1), Some(c2)) => c1 == c2,
            _ => false,
        }
    };

    queries
        .into_iter()
        .map(|q| {
            let a = q[0] as usize;
            let b = q[1] as usize;
            let mut c = q[2] as usize;
            let mut d = q[3] as usize;
            c = n - 1 - d;
            d = n - 1 - q[2] as usize;
            if a <= c {
                check(&pre1, &pre2, a, b, c, d, &diff, m)
            } else {
                check(&pre2, &pre1, c, d, a, b, &diff, m)
            }
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        can_make_palindrome_queries("abcabc".into(), vec![vec![1, 1, 3, 5], vec![0, 2, 5, 5]])
    );
}

#[cfg(test)]
mod tests {
    use super::can_make_palindrome_queries;

    #[test]
    fn example_one() {
        assert_eq!(
            can_make_palindrome_queries("abcabc".into(), vec![vec![1, 1, 3, 5], vec![0, 2, 5, 5]]),
            vec![true, true]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            can_make_palindrome_queries("abbcdecbba".into(), vec![vec![0, 2, 7, 9]]),
            vec![false]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            can_make_palindrome_queries("acbcab".into(), vec![vec![1, 2, 4, 5]]),
            vec![true]
        );
    }
}
