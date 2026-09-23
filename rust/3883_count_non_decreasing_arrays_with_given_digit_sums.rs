/// LeetCode #3883 - Count Non-Decreasing Arrays with Given Digit Sums
fn digit_sum(mut x: i32) -> i32 {
    if x == 0 {
        return 0;
    }
    let mut s = 0;
    while x > 0 {
        s += x % 10;
        x /= 10;
    }
    s
}

fn groups_by_digit_sum() -> Vec<Vec<i32>> {
    let mut groups = vec![Vec::new(); 51];
    for v in 0..=5000 {
        groups[digit_sum(v) as usize].push(v);
    }
    groups
}

fn count_arrays(digit_sums: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let groups = groups_by_digit_sum();
    let n = digit_sums.len();
    if n == 0 {
        return 0;
    }
    let mut vals = groups[digit_sums[0] as usize].clone();
    if vals.is_empty() {
        return 0;
    }
    let mut ways: Vec<i64> = vec![1; vals.len()];

    for i in 1..n {
        let cands = &groups[digit_sums[i] as usize];
        if cands.is_empty() {
            return 0;
        }
        let mut prefix = vec![0i64; vals.len()];
        prefix[0] = ways[0];
        for j in 1..vals.len() {
            prefix[j] = (prefix[j - 1] + ways[j]) % MOD;
        }
        let mut new_ways = vec![0i64; cands.len()];
        let mut p = 0usize;
        for (j, &c) in cands.iter().enumerate() {
            while p < vals.len() && vals[p] <= c {
                p += 1;
            }
            new_ways[j] = if p == 0 { 0 } else { prefix[p - 1] };
        }
        ways = new_ways;
        vals = cands.clone();
    }

    (ways.iter().sum::<i64>() % MOD) as i32
}

fn main() {
    println!("{}", count_arrays(vec![25, 1]));
}

#[cfg(test)]
mod tests {
    use super::count_arrays;

    #[test]
    fn example1() {
        assert_eq!(count_arrays(vec![25, 1]), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(count_arrays(vec![1]), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(count_arrays(vec![2, 49, 23]), 0);
    }
}
