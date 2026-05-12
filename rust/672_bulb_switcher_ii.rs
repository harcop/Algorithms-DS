/// LeetCode #672 - Bulb Switcher II
fn flip_lights(n: i32, presses: i32) -> i32 {
    if presses == 0 {
        return 1;
    }
    if n == 1 {
        return 2;
    }
    if n == 2 {
        return if presses == 1 { 3 } else { 4 };
    }
    match presses {
        1 => 4,
        2 => 7,
        _ => 8,
    }
}

fn main() {
    println!("{}", flip_lights(3, 2));
}

#[cfg(test)]
mod tests {
    use super::flip_lights;

    #[test]
    fn example_one() {
        assert_eq!(flip_lights(1, 1), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(flip_lights(2, 1), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(flip_lights(3, 1), 4);
    }

    #[test]
    fn example_four() {
        assert_eq!(flip_lights(3, 2), 7);
    }
}
