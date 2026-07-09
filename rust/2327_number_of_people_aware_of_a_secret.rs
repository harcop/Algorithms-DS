/// LeetCode #2327 - Number of People Aware of a Secret
fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
    let n = n as usize;
    let delay = delay as usize;
    let forget = forget as usize;
    let m = (n << 1) + 10;
    const MOD: i64 = 1_000_000_007;

    let mut d = vec![0i64; m];
    let mut cnt = vec![0i64; m];
    cnt[1] = 1;

    for i in 1..=n {
        if cnt[i] > 0 {
            d[i] = (d[i] + cnt[i]) % MOD;
            d[i + forget] = (d[i + forget] - cnt[i] + MOD) % MOD;
            let mut nxt = i + delay;
            while nxt < i + forget {
                cnt[nxt] = (cnt[nxt] + cnt[i]) % MOD;
                nxt += 1;
            }
        }
    }

    let mut ans = 0i64;
    for i in 1..=n {
        ans = (ans + d[i]) % MOD;
    }
    ans as i32
}

fn main() {
    println!("{}", people_aware_of_secret(6, 2, 4));
}

#[cfg(test)]
mod tests {
    use super::people_aware_of_secret;

    #[test]
    fn example_one() {
        assert_eq!(people_aware_of_secret(6, 2, 4), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(people_aware_of_secret(4, 1, 3), 6);
    }
}
