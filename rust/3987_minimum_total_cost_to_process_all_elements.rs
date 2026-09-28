/// LeetCode #3987 - Minimum Total Cost to Process All Elements
fn minimum_cost(nums: Vec<i32>, k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let k = k as i64;
    let mut cnt = 0i64;
    let mut cur = k;
    for x in nums {
        let x = x as i64;
        let diff = x - cur;
        if diff > 0 {
            let m = (diff + k - 1) / k;
            cur += m * k;
            cnt += m;
        }
        cur -= x;
    }
    cnt %= MOD;
    (cnt * (cnt + 1) / 2 % MOD) as i32
}

fn main() {
    println!("{}", minimum_cost(vec![1, 2, 3, 4], 4));
}

#[cfg(test)]
mod tests {
    use super::minimum_cost;

    #[test]
    fn example1() {
        assert_eq!(minimum_cost(vec![1, 2, 3, 4], 4), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_cost(vec![1, 1, 7, 14], 4), 15);
    }

    #[test]
    fn example3() {
        assert_eq!(minimum_cost(vec![1, 2, 3, 4], 10), 0);
    }
}
