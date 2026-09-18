/// LeetCode #3802 - Number of Ways to Paint Sheets (premium)
fn number_of_ways(n: i32, limit: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = n as i64;
    let m = limit.len();
    let l: Vec<i64> = limit.iter().map(|&x| (x as i64).min(n - 1)).collect();
    let t: Vec<i64> = l.iter().map(|&x| (n - x).max(1)).collect();
    let mut sorted_t = t.clone();
    sorted_t.sort_unstable();
    let mut pref = vec![0i64; m + 1];
    for i in 0..m {
        pref[i + 1] = pref[i] + sorted_t[i];
    }
    let mut ans = 0i64;
    for i in 0..m {
        let c = sorted_t.partition_point(|&x| x <= l[i]);
        let mut ways = c as i64 * (l[i] + 1) - pref[c];
        if t[i] <= l[i] {
            ways -= l[i] + 1 - t[i];
        }
        ans = (ans + ways) % MOD;
    }
    ans as i32
}

fn main() {
    println!("{}", number_of_ways(4, vec![3, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::number_of_ways;

    #[test]
    fn example1() {
        assert_eq!(number_of_ways(4, vec![3, 1, 2]), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(number_of_ways(3, vec![1, 2]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(number_of_ways(3, vec![2, 2]), 4);
    }
}
