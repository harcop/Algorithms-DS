/// LeetCode #372 - Super Pow (mod 1337)
const M: i64 = 1337;

fn powmod(mut a: i64, mut e: i64) -> i64 {
    a %= M;
    let mut r = 1i64;
    while e > 0 {
        if e & 1 == 1 {
            r = r * a % M;
        }
        a = a * a % M;
        e >>= 1;
    }
    r
}

fn helper(a: i64, exp: &[i32]) -> i64 {
    if exp.is_empty() {
        return 1;
    }
    let mut e = exp.to_vec();
    let last = e.pop().unwrap();
    let sub = helper(a, &e);
    powmod(sub, 10) * powmod(a, last as i64) % M
}

fn super_pow(a: i32, b: Vec<i32>) -> i32 {
    helper(a as i64, &b) as i32
}

fn main() {
    println!("{}", super_pow(2, vec![3]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        assert_eq!(super_pow(2, vec![3]), 8);
    }
}
