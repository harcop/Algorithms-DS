/// LeetCode #338 - Counting Bits
fn count_bits(n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut dp = vec![0; n + 1];
    for i in 1..=n {
        dp[i] = dp[i >> 1] + (i as i32 & 1);
    }
    dp
}

fn main() {
    println!("{:?}", count_bits(5));
}

#[cfg(test)]
mod tests {
    use super::count_bits;

    #[test]
    fn example_one() {
        assert_eq!(count_bits(2), vec![0,1,1]);
    }
}
