/// LeetCode #1041 - Robot Bounded In Circle
fn is_robot_bounded(instructions: String) -> bool {
    let mut x = 0i32;
    let mut y = 0i32;
    let mut dir = 0usize;
    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];
    for c in instructions.chars() {
        match c {
            'G' => {
                x += dx[dir];
                y += dy[dir];
            }
            'L' => dir = (dir + 3) % 4,
            'R' => dir = (dir + 1) % 4,
            _ => {}
        }
    }
    (x == 0 && y == 0) || dir != 0
}

fn main() {
    println!("{}", is_robot_bounded("GGLLGG".into()));
}

#[cfg(test)]
mod tests {
    use super::is_robot_bounded;

    #[test]
    fn example_one() {
        assert!(is_robot_bounded("GGLLGG".into()));
    }

    #[test]
    fn example_two() {
        assert!(!is_robot_bounded("GG".into()));
    }
}
