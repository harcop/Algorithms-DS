/// LeetCode #3260 - Find the Largest Palindrome Divisible by K
fn largest_palindrome(n: i32, k: i32) -> String {
    let n = n as usize;
    let k = k as usize;
    let mut p10 = vec![0usize; n];
    p10[0] = 1 % k;
    for i in 1..n {
        p10[i] = (p10[i - 1] * 10) % k;
    }
    let half = (n + 1) / 2;
    let mut contrib = vec![0usize; half];
    for i in 0..half {
        let j = n - 1 - i;
        contrib[i] = p10[j];
        if i != j {
            contrib[i] = (contrib[i] + p10[i]) % k;
        }
    }
    let mut can = vec![vec![false; k]; half + 1];
    can[half][0] = true;
    for pos in (0..half).rev() {
        let d0 = if pos == 0 { 1 } else { 0 };
        for d in d0..10 {
            let add = (d * contrib[pos]) % k;
            for r2 in 0..k {
                if can[pos + 1][r2] {
                    can[pos][(add + r2) % k] = true;
                }
            }
        }
    }
    let mut digits = vec![b'0'; n];
    let mut target = 0usize;
    for pos in 0..half {
        let d0 = if pos == 0 { 1 } else { 0 };
        for d in (d0..10).rev() {
            let add = (d * contrib[pos]) % k;
            let r2 = (target + k - add) % k;
            if can[pos + 1][r2] {
                digits[pos] = b'0' + d as u8;
                digits[n - 1 - pos] = b'0' + d as u8;
                target = r2;
                break;
            }
        }
    }
    String::from_utf8(digits).unwrap()
}

fn main() {
    println!("{}", largest_palindrome(3, 5));
}

#[cfg(test)]
mod tests {
    use super::largest_palindrome;

    #[test]
    fn example1() {
        assert_eq!(largest_palindrome(3, 5), "595");
    }

    #[test]
    fn example2() {
        assert_eq!(largest_palindrome(1, 4), "8");
    }

    #[test]
    fn example3() {
        assert_eq!(largest_palindrome(5, 6), "89898");
    }
}
