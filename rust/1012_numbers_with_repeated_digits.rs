/// LeetCode #1012 - Numbers With Repeated Digits
fn num_dup_digits_at_most_n(n: i32) -> i32 {
    let s = n.to_string();
    let digits: Vec<i32> = s.bytes().map(|c| (c - b'0') as i32).collect();
    let len = digits.len();
    let mut total = 0i32;
    for l in 1..len {
        total += 9 * perm(9, l - 1);
    }
    let mut used = [false; 10];
    for i in 0..len {
        for d in 0..digits[i] {
            if d == 0 && i == 0 {
                continue;
            }
            if used[d as usize] {
                continue;
            }
            total += perm(10 - (i + 1), len - i - 1);
        }
        if used[digits[i] as usize] {
            return n - total;
        }
        used[digits[i] as usize] = true;
    }
    n - total
}

fn perm(n: i32, k: i32) -> i32 {
    if k < 0 {
        return 0;
    }
    let mut res = 1i32;
    for i in 0..k {
        res *= n - i;
    }
    res
}

fn main() {
    println!("{}", num_dup_digits_at_most_n(100));
}

#[cfg(test)]
mod tests {
    use super::num_dup_digits_at_most_n;

    #[test]
    fn example_one() {
        assert_eq!(num_dup_digits_at_most_n(20), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_dup_digits_at_most_n(100), 10);
    }
}
