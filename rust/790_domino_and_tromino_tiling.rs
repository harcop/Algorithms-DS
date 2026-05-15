/// LeetCode #790 - Domino and Tromino Tiling
const MOD: i64 = 1_000_000_007;

fn num_tilings(n: i32) -> i32 {
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return 1;
    }
    if n == 2 {
        return 2;
    }
    let mut f0 = 1i64;
    let mut f1 = 1i64;
    let mut f2 = 2i64;
    for _ in 3..=n {
        let f3 = (2 * f2 + f0) % MOD;
        f0 = f1;
        f1 = f2;
        f2 = f3;
    }
    f2 as i32
}

fn main() {
    println!("{}", num_tilings(3));
}

#[cfg(test)]
mod tests {
    use super::num_tilings;

    #[test]
    fn example_one() {
        assert_eq!(num_tilings(3), 5);
    }
}
