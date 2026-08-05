/// LeetCode #3014 - Minimum Number of Pushes to Type Word I
fn minimum_pushes(word: String) -> i32 {
    let n = word.len();
    let mut ans = 0;
    let mut k = 1;
    for _ in 0..n / 8 {
        ans += k * 8;
        k += 1;
    }
    ans += k * (n % 8);
    ans as i32
}

fn main() {
    println!("{}", minimum_pushes("abcde".into()));
    println!("{}", minimum_pushes("xycdefghij".into()));
}

#[cfg(test)]
mod tests {
    use super::minimum_pushes;

    #[test]
    fn example_one() {
        assert_eq!(minimum_pushes("abcde".into()), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_pushes("xycdefghij".into()), 12);
    }
}
