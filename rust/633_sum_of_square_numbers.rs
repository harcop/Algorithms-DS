/// LeetCode #633 - Sum of Square Numbers
fn judge_square_sum(c: i32) -> bool {
    let c = c as i64;
    let mut a: i64 = 0;
    let mut b: i64 = (c as f64).sqrt() as i64;
    while a <= b {
        let s = a * a + b * b;
        if s == c { return true; }
        if s < c { a += 1; } else { b -= 1; }
    }
    false
}

fn main() {
    println!("{}", judge_square_sum(5));
}

#[cfg(test)]
mod tests {
    use super::judge_square_sum;

    #[test]
    fn example_one() {
        assert!(judge_square_sum(5));
    }

    #[test]
    fn example_two() {
        assert!(!judge_square_sum(3));
    }
}
