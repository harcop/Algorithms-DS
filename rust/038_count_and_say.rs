/// LeetCode #38 - Count and Say
fn count_and_say(n: i32) -> String {
    if n <= 0 {
        return String::new();
    }
    let mut s = "1".to_string();
    for _ in 1..n {
        s = next_count_say(&s);
    }
    s
}

fn next_count_say(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = String::new();
    let mut i = 0usize;
    while i < b.len() {
        let mut j = i;
        while j < b.len() && b[j] == b[i] {
            j += 1;
        }
        out.push_str(&((j - i) as u32).to_string());
        out.push(b[i] as char);
        i = j;
    }
    out
}

fn main() {
    println!("{}", count_and_say(4));
}

#[cfg(test)]
mod tests {
    use super::count_and_say;

    #[test]
    fn example_one() {
        assert_eq!(count_and_say(1), "1");
    }

    #[test]
    fn example_two() {
        assert_eq!(count_and_say(4), "1211");
    }
}
