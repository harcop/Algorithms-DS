/// LeetCode #3801 - Minimum Cost to Merge Sorted Lists
fn min_merge_cost(lists: Vec<Vec<i32>>) -> i64 {
    let n = lists.len();
    let mut vals: Vec<i32> = lists.iter().flatten().copied().collect();
    vals.sort_unstable();
    vals.dedup();
    let m = 1 << n;
    let mut cnt = vec![0usize; m];
    let mut med = vec![0i32; m];
    for i in 1..m {
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                cnt[i] += lists[j].len();
            }
        }
        let need = (cnt[i] + 1) / 2;
        let mut l = 0;
        let mut r = vals.len() - 1;
        while l < r {
            let mid = (l + r) / 2;
            let mut le = 0;
            let mut b = i;
            while b > 0 {
                let id = b.trailing_zeros() as usize;
                le += lists[id].partition_point(|&x| x <= vals[mid]);
                if le >= need {
                    break;
                }
                b &= b - 1;
            }
            if le >= need {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        med[i] = vals[l];
    }
    let mut f = vec![i64::MAX / 4; m];
    for i in 1..m {
        if i.count_ones() == 1 {
            f[i] = 0;
            continue;
        }
        let mut j = (i - 1) & i;
        while j > 0 {
            let k = i ^ j;
            if j <= k {
                f[i] = f[i].min(f[j] + f[k] + (med[j] - med[k]).abs() as i64);
            }
            j = (j - 1) & i;
        }
        f[i] += cnt[i] as i64;
    }
    f[m - 1]
}

fn main() {
    println!(
        "{}",
        min_merge_cost(vec![vec![1, 3, 5], vec![2, 4], vec![6, 7, 8]])
    );
}

#[cfg(test)]
mod tests {
    use super::min_merge_cost;

    #[test]
    fn example1() {
        assert_eq!(
            min_merge_cost(vec![vec![1, 3, 5], vec![2, 4], vec![6, 7, 8]]),
            18
        );
    }

    #[test]
    fn example2() {
        assert_eq!(min_merge_cost(vec![vec![1, 1, 5], vec![1, 4, 7, 8]]), 10);
    }

    #[test]
    fn example3() {
        assert_eq!(min_merge_cost(vec![vec![1], vec![3]]), 4);
    }

    #[test]
    fn example4() {
        assert_eq!(min_merge_cost(vec![vec![1], vec![1]]), 2);
    }
}
