/// LeetCode #2803 - Factorial Generator (JS problem; Rust analogue)
fn factorial(n: i64) -> Vec<i64> {
    if n == 0 {
        return vec![1];
    }
    let mut ans = Vec::with_capacity(n as usize);
    let mut cur = 1i64;
    for i in 1..=n {
        cur *= i;
        ans.push(cur);
    }
    ans
}

fn main() {
    println!("{:?}", factorial(5));
}

#[cfg(test)]
mod tests {
    use super::factorial;

    #[test]
    fn example_one() {
        assert_eq!(factorial(5), vec![1, 2, 6, 24, 120]);
    }

    #[test]
    fn example_two() {
        assert_eq!(factorial(2), vec![1, 2]);
    }

    #[test]
    fn example_three() {
        assert_eq!(factorial(0), vec![1]);
    }
}
