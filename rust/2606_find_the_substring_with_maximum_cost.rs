/// LeetCode #2606 - Find the Substring With Maximum Cost
fn maximum_cost_substring(s: String, chars: String, vals: Vec<i32>) -> i32 {
    let mut d: Vec<i32> = (1..=26).collect();
    for (i, c) in chars.bytes().enumerate() {
        d[(c - b'a') as usize] = vals[i];
    }
    let mut ans = 0;
    let mut f = 0;
    for c in s.bytes() {
        f = f.max(0) + d[(c - b'a') as usize];
        ans = ans.max(f);
    }
    ans
}

fn main() {
    println!(
        "{}",
        maximum_cost_substring("adaa".to_string(), "d".to_string(), vec![-1000])
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_cost_substring;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_cost_substring("adaa".to_string(), "d".to_string(), vec![-1000]),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            maximum_cost_substring("abc".to_string(), "abc".to_string(), vec![-1, -1, -1]),
            0
        );
    }
}
