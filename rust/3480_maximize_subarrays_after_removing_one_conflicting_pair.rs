/// LeetCode #3480 - Maximize Subarrays After Removing One Conflicting Pair
fn max_subarrays(n: i32, conflicting_pairs: Vec<Vec<i32>>) -> i64 {
    let n = n as usize;
    let mut g = vec![Vec::new(); n + 1];
    for p in &conflicting_pairs {
        let mut a = p[0] as usize;
        let mut b = p[1] as usize;
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        g[a].push(b);
    }
    let mut cnt = vec![0i64; n + 2];
    let mut ans = 0i64;
    let mut add = 0i64;
    let mut b1 = n + 1;
    let mut b2 = n + 1;
    for a in (1..=n).rev() {
        for &b in &g[a] {
            if b < b1 {
                b2 = b1;
                b1 = b;
            } else if b < b2 {
                b2 = b;
            }
        }
        ans += (b1 - a) as i64;
        cnt[b1] += (b2 - b1) as i64;
        add = add.max(cnt[b1]);
    }
    ans + add
}

fn main() {
    println!("{}", max_subarrays(4, vec![vec![2, 3], vec![1, 4]]));
}

#[cfg(test)]
mod tests {
    use super::max_subarrays;

    #[test]
    fn example1() {
        assert_eq!(max_subarrays(4, vec![vec![2, 3], vec![1, 4]]), 9);
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_subarrays(5, vec![vec![1, 2], vec![2, 5], vec![3, 5]]),
            12
        );
    }
}
