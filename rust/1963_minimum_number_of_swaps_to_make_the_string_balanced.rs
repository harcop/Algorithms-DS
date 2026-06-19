/// LeetCode #1963 - Minimum Number of Swaps to Make the String Balanced
fn min_swaps(s: String) -> i32 {
    let mut x = 0i32;
    for b in s.bytes() {
        if b == b'[' {
            x += 1;
        } else if x > 0 {
            x -= 1;
        }
    }
    (x + 1) / 2
}

fn main() {
    println!("{}", min_swaps("][][".into()));
}

#[cfg(test)]
mod tests {
    use super::min_swaps;

    #[test]
    fn example_one() {
        assert_eq!(min_swaps("][][".into()), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_swaps("[][]".into()), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(min_swaps("[[[]]][]".into()), 0);
    }
}
