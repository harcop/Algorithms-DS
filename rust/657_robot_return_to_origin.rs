/// LeetCode #657 - Robot Return to Origin
fn judge_circle(moves: String) -> bool {
    let mut x = 0i32;
    let mut y = 0i32;
    for c in moves.chars() {
        match c {
            'U' => y += 1,
            'D' => y -= 1,
            'L' => x -= 1,
            'R' => x += 1,
            _ => {}
        }
    }
    x == 0 && y == 0
}

fn main() {
    println!("{}", judge_circle("UD".into()));
}

#[cfg(test)]
mod tests {
    use super::judge_circle;

    #[test]
    fn example_one() {
        assert!(judge_circle("UD".into()));
    }

    #[test]
    fn example_two() {
        assert!(!judge_circle("LL".into()));
    }
}
