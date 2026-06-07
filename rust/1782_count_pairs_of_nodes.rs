/// LeetCode #1782 - Count Pairs Of Nodes
use std::collections::HashMap;

fn count_pairs(n: i32, edges: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let n = n as usize;
    let mut cnt = vec![0i32; n];
    let mut g: HashMap<(usize, usize), i32> = HashMap::new();
    for e in edges {
        let mut a = (e[0] - 1) as usize;
        let mut b = (e[1] - 1) as usize;
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        cnt[a] += 1;
        cnt[b] += 1;
        *g.entry((a, b)).or_insert(0) += 1;
    }
    let mut s = cnt.clone();
    s.sort_unstable();
    queries
        .into_iter()
        .map(|t| {
            let mut ans = 0i32;
            for (j, &x) in s.iter().enumerate() {
                let k = bisect_right(&s, t - x, j + 1);
                ans += (n - k) as i32;
            }
            for (&(a, b), &v) in &g {
                if cnt[a] + cnt[b] > t && cnt[a] + cnt[b] - v <= t {
                    ans -= 1;
                }
            }
            ans
        })
        .collect()
}

fn bisect_right(s: &[i32], x: i32, lo: usize) -> usize {
    let mut l = lo;
    let mut r = s.len();
    while l < r {
        let m = (l + r) / 2;
        if s[m] <= x {
            l = m + 1;
        } else {
            r = m;
        }
    }
    l
}

fn main() {
    println!(
        "{:?}",
        count_pairs(
            4,
            vec![vec![1, 2], vec![2, 4], vec![1, 3], vec![2, 3], vec![2, 1]],
            vec![2, 3],
        )
    );
}
#[cfg(test)]
mod tests {
    use super::count_pairs;
    #[test]
    fn example_one() {
        assert_eq!(
            count_pairs(
                4,
                vec![vec![1, 2], vec![2, 4], vec![1, 3], vec![2, 3], vec![2, 1]],
                vec![2, 3],
            ),
            vec![6, 5]
        );
    }
    #[test]
    fn example_two() {
        assert_eq!(
            count_pairs(
                5,
                vec![
                    vec![1, 5],
                    vec![1, 5],
                    vec![3, 4],
                    vec![2, 5],
                    vec![1, 3],
                    vec![5, 1],
                    vec![2, 3],
                    vec![2, 5],
                ],
                vec![1, 2, 3, 4, 5],
            ),
            vec![10, 10, 9, 8, 6]
        );
    }
}
