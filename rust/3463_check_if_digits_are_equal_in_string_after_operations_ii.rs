/// LeetCode #3463 - Check If Digits Are Equal in String After Operations II
fn nck(n: i32, k: i32) -> i32 {
    if k < 0 || k > n {
        return 0;
    }
    let mut res = 1;
    for i in 0..k {
        res = res * (n - i) / (i + 1);
    }
    res
}

fn lucas(mut n: i32, mut k: i32, prime: i32) -> i32 {
    let mut res = 1;
    while n > 0 || k > 0 {
        let n_mod = n % prime;
        let k_mod = k % prime;
        res = (res * nck(n_mod, k_mod)) % prime;
        n /= prime;
        k /= prime;
    }
    res
}

fn nck_mod10(n: i32, k: i32) -> i32 {
    let mod2 = lucas(n, k, 2);
    let mod5 = lucas(n, k, 5);
    let lookup = [[0, 6, 2, 8, 4], [5, 1, 7, 3, 9]];
    lookup[mod2 as usize][mod5 as usize]
}

fn has_same_digits(s: String) -> bool {
    let n = s.len() as i32;
    let b = s.as_bytes();
    let mut num1 = 0;
    let mut num2 = 0;
    for i in 0..n - 1 {
        let coef = nck_mod10(n - 2, i);
        num1 = (num1 + coef * (b[i as usize] - b'0') as i32) % 10;
        num2 = (num2 + coef * (b[i as usize + 1] - b'0') as i32) % 10;
    }
    num1 == num2
}

fn main() {
    println!("{}", has_same_digits("3902".into()));
}

#[cfg(test)]
mod tests {
    use super::has_same_digits;

    #[test]
    fn example1() {
        assert!(has_same_digits("3902".into()));
    }

    #[test]
    fn example2() {
        assert!(!has_same_digits("34789".into()));
    }
}
