/// LeetCode #541 - Reverse String II
fn reverse_str(s: String, k: i32) -> String {
    let k = k as usize;
    let mut v: Vec<char> = s.chars().collect();
    let n = v.len();
    let mut i = 0usize;
    while i < n {
        let j = (i + k).min(n);
        v[i..j].reverse();
        i += 2 * k;
    }
    v.into_iter().collect()
}

fn main() {
    println!("{}", reverse_str("abcdefg".into(), 2));
}

#[cfg(test)]
mod tests {
    use super::reverse_str;

    #[test]
    fn example_one() {
        assert_eq!(reverse_str("abcdefg".into(), 2), "bacdfeg");
    }
}
