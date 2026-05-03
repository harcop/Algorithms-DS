/// LeetCode #241 - Different Ways to Add Parentheses
fn diff_ways_to_compute(input: String) -> Vec<i32> {
    let b = input.as_bytes();
    if !b.iter().any(|&c| c == b'+' || c == b'-' || c == b'*') {
        return vec![input.parse().unwrap()];
    }
    let mut out = vec![];
    for i in 0..b.len() {
        let c = b[i] as char;
        if c == '+' || c == '-' || c == '*' {
            let left = diff_ways_to_compute(String::from_utf8(b[..i].to_vec()).unwrap());
            let right = diff_ways_to_compute(String::from_utf8(b[i + 1..].to_vec()).unwrap());
            for a in &left {
                for bb in &right {
                    out.push(match c {
                        '+' => a + bb,
                        '-' => a - bb,
                        _ => a * bb,
                    });
                }
            }
        }
    }
    out
}

fn main() {
    println!("{:?}", diff_ways_to_compute("2-1-1".into()));
}

#[cfg(test)]
mod tests {
    use super::diff_ways_to_compute;

    #[test]
    fn example_one() {
        let mut v = diff_ways_to_compute("2-1-1".into());
        v.sort();
        assert_eq!(v, vec![0, 2]);
    }

    #[test]
    fn example_two() {
        let mut v = diff_ways_to_compute("2*3-4*5".into());
        v.sort();
        let mut e = vec![-34, -14, -10, -10, 10];
        e.sort();
        assert_eq!(v, e);
    }
}
