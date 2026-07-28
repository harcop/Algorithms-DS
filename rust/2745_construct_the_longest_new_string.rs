/// LeetCode #2745 - Construct the Longest New String
fn longest_string(x: i32, y: i32, z: i32) -> i32 {
    if x < y {
        (x * 2 + z + 1) * 2
    } else if x > y {
        (y * 2 + z + 1) * 2
    } else {
        (x + y + z) * 2
    }
}

fn main() {
    println!("{}", longest_string(2, 5, 1));
}

#[cfg(test)]
mod tests {
    use super::longest_string;

    #[test]
    fn example_one() {
        assert_eq!(longest_string(2, 5, 1), 12);
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_string(3, 2, 2), 14);
    }
}
