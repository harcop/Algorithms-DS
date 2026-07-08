/// LeetCode #2305 - Fair Distribution of Cookies
fn distribute_cookies(mut cookies: Vec<i32>, k: i32) -> i32 {
    cookies.sort_by(|a, b| b.cmp(a));
    let k = k as usize;
    let mut cnt = vec![0i32; k];
    let mut ans = i32::MAX;

    fn dfs(i: usize, cookies: &[i32], cnt: &mut [i32], ans: &mut i32, k: usize) {
        if i >= cookies.len() {
            *ans = (*ans).min(*cnt.iter().max().unwrap());
            return;
        }
        for j in 0..k {
            if cnt[j] + cookies[i] >= *ans || (j > 0 && cnt[j] == cnt[j - 1]) {
                continue;
            }
            cnt[j] += cookies[i];
            dfs(i + 1, cookies, cnt, ans, k);
            cnt[j] -= cookies[i];
        }
    }

    dfs(0, &cookies, &mut cnt, &mut ans, k);
    ans
}

fn main() {
    println!("{}", distribute_cookies(vec![8, 15, 10, 20, 8], 2));
}

#[cfg(test)]
mod tests {
    use super::distribute_cookies;

    #[test]
    fn example_one() {
        assert_eq!(distribute_cookies(vec![8, 15, 10, 20, 8], 2), 31);
    }

    #[test]
    fn example_two() {
        assert_eq!(distribute_cookies(vec![6, 1, 3, 2, 2, 4, 1, 2], 3), 7);
    }
}
