/// LeetCode #1513 - Number Of Substrings With Only 1s
const MOD: i64 = 1_000_000_007;

fn num_sub(s: String) -> i32 {
    let mut ans = 0i64;
    let mut run = 0i64;
    for c in s.bytes() {
        if c == b'1' {
            run += 1;
            ans = (ans + run) % MOD;
        } else {
            run = 0;
        }
    }
    ans as i32
}

fn main() {
    println!("{}", num_sub("0110111".into()));
}

#[cfg(test)]
mod tests {
    use super::num_sub;

    #[test]
    fn example_one() {
        assert_eq!(num_sub("0110111".into()), 9);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_sub("101".into()), 2);
    }
}
