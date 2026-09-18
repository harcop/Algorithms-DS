/// LeetCode #3792 - Sum of Increasing Product Blocks (premium)
fn sum_of_blocks(n: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut ans = 0i64;
    let mut k = 1i64;
    for i in 1..=n as i64 {
        let mut x = 1i64;
        for j in k..k + i {
            x = x * j % MOD;
        }
        ans = (ans + x) % MOD;
        k += i;
    }
    ans as i32
}

fn main() {
    println!("{}", sum_of_blocks(3));
}

#[cfg(test)]
mod tests {
    use super::sum_of_blocks;

    #[test]
    fn example1() {
        assert_eq!(sum_of_blocks(3), 127);
    }

    #[test]
    fn example2() {
        assert_eq!(sum_of_blocks(7), 6997165);
    }
}
