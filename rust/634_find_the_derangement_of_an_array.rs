/// LeetCode #634 - Find the Derangement of an Array
fn find_derangement(n: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return 0;
    }
    let mut a = 1i64;
    let mut b = 0i64;
    for i in 2..=n as i64 {
        let c = ((i - 1) * ((a + b) % MOD)) % MOD;
        a = b;
        b = c;
    }
    b as i32
}

fn main() {
    println!("{}", find_derangement(3));
}

#[cfg(test)]
mod tests {
    use super::find_derangement;

    #[test]
    fn example_one() {
        assert_eq!(find_derangement(3), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_derangement(2), 1);
    }
}
