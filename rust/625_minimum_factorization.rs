/// LeetCode #625 - Minimum Factorization
fn smallest_factorization(num: i32) -> i32 {
    if num < 10 {
        return num;
    }
    let mut n = num;
    let mut digits = vec![];
    for d in (2..=9).rev() {
        while n % d == 0 {
            digits.push(d);
            n /= d;
        }
    }
    if n > 1 {
        return 0;
    }
    digits.sort();
    let mut ans: i64 = 0;
    for d in digits {
        ans = ans * 10 + d as i64;
        if ans > i32::MAX as i64 {
            return 0;
        }
    }
    ans as i32
}

fn main() {
    println!("{}", smallest_factorization(48));
}

#[cfg(test)]
mod tests {
    use super::smallest_factorization;

    #[test]
    fn example_one() {
        assert_eq!(smallest_factorization(48), 68);
    }

    #[test]
    fn example_two() {
        assert_eq!(smallest_factorization(15), 35);
    }
}
