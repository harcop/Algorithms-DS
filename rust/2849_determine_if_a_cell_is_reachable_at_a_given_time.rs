/// LeetCode #2849 - Determine if a Cell Is Reachable at a Given Time
fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
    let distance = (sx - fx).abs().max((sy - fy).abs());
    if distance == 0 {
        t != 1
    } else {
        distance <= t
    }
}

fn main() {
    println!("{}", is_reachable_at_time(2, 4, 7, 7, 6));
}

#[cfg(test)]
mod tests {
    use super::is_reachable_at_time;

    #[test]
    fn example_one() {
        assert!(is_reachable_at_time(2, 4, 7, 7, 6));
    }

    #[test]
    fn example_two() {
        assert!(!is_reachable_at_time(3, 1, 7, 3, 3));
    }
}
