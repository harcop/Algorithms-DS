/// LeetCode #2939 - Maximum Xor Product
fn maximum_xor_product(a: i64, b: i64, n: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut ax = (a >> n) << n;
    let mut bx = (b >> n) << n;
    for i in (0..n).rev() {
        let x = (a >> i) & 1;
        let y = (b >> i) & 1;
        if x == y {
            ax |= 1 << i;
            bx |= 1 << i;
        } else if ax > bx {
            bx |= 1 << i;
        } else {
            ax |= 1 << i;
        }
    }
    ((ax % MOD) * (bx % MOD) % MOD) as i32
}

fn main() {
    println!("{}", maximum_xor_product(12, 5, 4));
}

#[cfg(test)]
mod tests {
    use super::maximum_xor_product;

    #[test]
    fn example_one() {
        assert_eq!(maximum_xor_product(12, 5, 4), 98);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_xor_product(6, 7, 5), 930);
    }

    #[test]
    fn example_three() {
        assert_eq!(maximum_xor_product(1, 6, 3), 12);
    }
}
