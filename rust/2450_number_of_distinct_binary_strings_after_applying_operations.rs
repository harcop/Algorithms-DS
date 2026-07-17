/// LeetCode #2450 - Number of Distinct Binary Strings After Applying Operations
fn count_distinct_strings(s: String, k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;

    let mut exponent = s.len() as i32 - k + 1;
    let mut base = 2i64;
    let mut answer = 1i64;

    while exponent > 0 {
        if exponent & 1 == 1 {
            answer = answer * base % MOD;
        }
        base = base * base % MOD;
        exponent >>= 1;
    }

    answer as i32
}

fn main() {
    println!("{}", count_distinct_strings("1001".to_string(), 3));
}

#[cfg(test)]
mod tests {
    use super::count_distinct_strings;

    #[test]
    fn two_independent_operations() {
        assert_eq!(count_distinct_strings("1001".to_string(), 3), 4);
    }

    #[test]
    fn full_string_operation() {
        assert_eq!(count_distinct_strings("101".to_string(), 3), 2);
    }
}
