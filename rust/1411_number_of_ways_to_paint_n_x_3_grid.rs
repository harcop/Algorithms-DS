/// LeetCode #1411 - Number Of Ways To Paint N X 3 Grid
fn num_of_ways(n: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = n as usize;
    let mut a = 6i64;
    let mut b = 6i64;
    for _ in 1..n {
        let na = (a * 3 + b * 2) % MOD;
        let nb = (a * 2 + b * 2) % MOD;
        a = na;
        b = nb;
    }
    ((a + b) % MOD) as i32
}

fn main() {
    println!("{}", num_of_ways(7));
}

#[cfg(test)]
mod tests {
    use super::num_of_ways;

    #[test]
    fn example_one() {
        assert_eq!(num_of_ways(1), 12);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_of_ways(7), 106494);
    }
}

