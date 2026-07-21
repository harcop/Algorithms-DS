/// LeetCode #2543 - Check if Point Is Reachable
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn is_reachable(target_x: i32, target_y: i32) -> bool {
    gcd(target_x, target_y).count_ones() == 1
}

fn main() {
    println!("{}", is_reachable(4, 7));
}

#[cfg(test)]
mod tests {
    use super::is_reachable;

    #[test]
    fn example_one() {
        assert!(!is_reachable(6, 9));
    }

    #[test]
    fn example_two() {
        assert!(is_reachable(4, 7));
    }
}
