/// LeetCode #779 - K-th Symbol in Grammar
fn kth_grammar(n: i32, k: i32) -> i32 {
    let mut k = k - 1;
    let mut ans = 0i32;
    for _ in 0..n - 1 {
        ans ^= (k & 1) as i32;
        k >>= 1;
    }
    ans
}

fn main() {
    println!("{}", kth_grammar(4, 5));
}

#[cfg(test)]
mod tests {
    use super::kth_grammar;

    #[test]
    fn example_one() {
        assert_eq!(kth_grammar(1, 1), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(kth_grammar(2, 1), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(kth_grammar(2, 2), 1);
    }
}
