/// LeetCode #3248 - Snake in Matrix
fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    for c in commands {
        match c.as_str() {
            "UP" => x -= 1,
            "DOWN" => x += 1,
            "LEFT" => y -= 1,
            "RIGHT" => y += 1,
            _ => {}
        }
    }
    x * n + y
}

fn main() {
    println!(
        "{}",
        final_position_of_snake(2, vec!["RIGHT".into(), "DOWN".into()])
    );
}

#[cfg(test)]
mod tests {
    use super::final_position_of_snake;

    #[test]
    fn example1() {
        assert_eq!(
            final_position_of_snake(2, vec!["RIGHT".into(), "DOWN".into()]),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            final_position_of_snake(3, vec!["DOWN".into(), "RIGHT".into(), "UP".into()]),
            1
        );
    }
}
