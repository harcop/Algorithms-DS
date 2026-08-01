/// LeetCode #2844 - Minimum Operations to Make a Special Number
fn minimum_operations(num: String) -> i32 {
    let n = num.len();
    let bytes = num.as_bytes();
    let mut memo = vec![vec![-1; 25]; n];

    fn dfs(i: usize, k: usize, n: usize, bytes: &[u8], memo: &mut Vec<Vec<i32>>) -> i32 {
        if i == n {
            return if k == 0 { 0 } else { n as i32 };
        }
        if memo[i][k] != -1 {
            return memo[i][k];
        }
        let mut result = dfs(i + 1, k, n, bytes, memo) + 1;
        let digit = (bytes[i] - b'0') as usize;
        let next_remainder = (k * 10 + digit) % 25;
        result = result.min(dfs(i + 1, next_remainder, n, bytes, memo));
        memo[i][k] = result;
        result
    }

    dfs(0, 0, n, bytes, &mut memo)
}

fn main() {
    println!("{}", minimum_operations("2245047".into()));
}

#[cfg(test)]
mod tests {
    use super::minimum_operations;

    #[test]
    fn example_one() {
        assert_eq!(minimum_operations("2245047".into()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_operations("2908305".into()), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_operations("10".into()), 1);
    }
}
