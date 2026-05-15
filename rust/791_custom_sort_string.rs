/// LeetCode #791 - Custom Sort String
fn custom_sort_string(order: String, s: String) -> String {
    let mut cnt = [0usize; 26];
    for c in s.bytes() {
        cnt[(c - b'a') as usize] += 1;
    }
    let mut out = String::new();
    for c in order.bytes() {
        let i = (c - b'a') as usize;
        for _ in 0..cnt[i] {
            out.push(c as char);
        }
        cnt[i] = 0;
    }
    for i in 0..26 {
        for _ in 0..cnt[i] {
            out.push((b'a' + i as u8) as char);
        }
    }
    out
}

fn main() {
    println!("{}", custom_sort_string("cba".into(), "abcd".into()));
}

#[cfg(test)]
mod tests {
    use super::custom_sort_string;

    #[test]
    fn example_one() {
        assert_eq!(custom_sort_string("cba".into(), "abcd".into()), "cbad");
    }
}
