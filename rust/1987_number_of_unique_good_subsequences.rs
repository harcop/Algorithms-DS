/// LeetCode #1987 - Number of Unique Good Subsequences
const MOD: i64 = 1_000_000_007;

fn number_of_unique_good_subsequences(binary: String) -> i32 {
    let mut f = 0i64;
    let mut g = 0i64;
    let mut ans = 0i64;
    for c in binary.bytes() {
        if c == b'0' {
            g = (g + f) % MOD;
            ans = 1;
        } else {
            f = (f + g + 1) % MOD;
        }
    }
    ((ans + f + g) % MOD) as i32
}

fn main() {
    println!("{}", number_of_unique_good_subsequences("001".into()));
}

#[cfg(test)]
mod tests {
    use super::number_of_unique_good_subsequences;

    #[test]
    fn example_one() {
        assert_eq!(number_of_unique_good_subsequences("001".into()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_unique_good_subsequences("11".into()), 2);
    }
}
