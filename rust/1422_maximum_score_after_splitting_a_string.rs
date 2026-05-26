/// LeetCode #1422 - Maximum Score After Splitting A String
fn max_score(s: String) -> i32 {
    let b = s.as_bytes();
    let mut right_ones = b.iter().filter(|&&c| c == b'1').count() as i32;
    let mut left_zeros = 0i32;
    let mut best = 0i32;
    for i in 0..b.len() - 1 {
        if b[i] == b'0' {
            left_zeros += 1;
        } else {
            right_ones -= 1;
        }
        best = best.max(left_zeros * right_ones);
    }
    best
}

fn main() {
    println!("{}", max_score("011101".into()));
}

#[cfg(test)]
mod tests {
    use super::max_score;

    #[test]
    fn example_one() {
        assert_eq!(max_score("011101".into()), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_score("00101100".into()), 6);
    }
}

