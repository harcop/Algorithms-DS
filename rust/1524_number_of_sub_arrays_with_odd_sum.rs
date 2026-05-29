/// LeetCode #1524 - Number Of Sub Arrays With Odd Sum
const MOD: i64 = 1_000_000_007;

fn num_of_subarrays(arr: Vec<i32>) -> i32 {
    let mut odd = 0i64;
    let mut even = 1i64;
    let mut sum = 0i64;
    let mut ans = 0i64;
    for x in arr {
        sum += x as i64;
        if sum % 2 == 0 {
            ans = (ans + odd) % MOD;
            even += 1;
        } else {
            ans = (ans + even) % MOD;
            odd += 1;
        }
    }
    ans as i32
}

fn main() {
    println!("{}", num_of_subarrays(vec![1, 3, 5]));
}

#[cfg(test)]
mod tests {
    use super::num_of_subarrays;

    #[test]
    fn example_one() {
        assert_eq!(num_of_subarrays(vec![1, 3, 5]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_of_subarrays(vec![2, 4, 6]), 0);
    }
}
