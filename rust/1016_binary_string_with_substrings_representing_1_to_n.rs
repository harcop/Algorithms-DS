/// LeetCode #1016 - Binary String With Substrings Representing 1 To N
fn query_string(s: String, n: i32) -> bool {
    let n = n as usize;
    let len = (n as f64).log2().ceil() as usize + 1;
    for x in 1..=n {
        let bits = format!("{:b}", x);
        if !s.contains(&bits) {
            return false;
        }
    }
    let _ = len;
    true
}

fn main() {
    println!("{}", query_string("0110".into(), 3));
}

#[cfg(test)]
mod tests {
    use super::query_string;

    #[test]
    fn example_one() {
        assert!(query_string("0110".into(), 3));
    }

    #[test]
    fn example_two() {
        assert!(!query_string("0110".into(), 4));
    }
}
