/// LeetCode #2269 - Find the K-Beauty of a Number
fn divisor_substrings(num: i32, k: i32) -> i32 {
    let s = num.to_string();
    let k = k as usize;
    let mut ans = 0;

    for i in 0..=s.len().saturating_sub(k) {
        let x: i32 = s[i..i + k].parse().unwrap();
        if x != 0 && num % x == 0 {
            ans += 1;
        }
    }

    ans
}

fn main() {
    println!("{}", divisor_substrings(240, 2));
}

#[cfg(test)]
mod tests {
    use super::divisor_substrings;

    #[test]
    fn example_one() {
        assert_eq!(divisor_substrings(240, 2), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(divisor_substrings(430043, 2), 2);
    }
}
