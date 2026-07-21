/// LeetCode #2550 - Count Collisions of Monkeys on a Polygon
const MOD: i64 = 1_000_000_007;

fn mod_pow(mut x: i64, mut n: i32) -> i64 {
    let mut result = 1i64;
    x %= MOD;
    while n > 0 {
        if n % 2 == 1 {
            result = result * x % MOD;
        }
        x = x * x % MOD;
        n /= 2;
    }
    result
}

fn monkey_move(n: i32) -> i32 {
    let res = mod_pow(2, n) - 2;
    if res < 0 {
        (res + MOD) as i32
    } else {
        res as i32
    }
}

fn main() {
    println!("{}", monkey_move(3));
}

#[cfg(test)]
mod tests {
    use super::monkey_move;

    #[test]
    fn example_one() {
        assert_eq!(monkey_move(3), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(monkey_move(4), 14);
    }
}
