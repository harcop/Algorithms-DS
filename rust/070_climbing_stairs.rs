/// LeetCode #70 - Climbing Stairs
fn climb_stairs(n: i32) -> i32 {
    let n = n as usize;
    if n <= 2 {
        return n as i32;
    }
    let mut a = 1;
    let mut b = 2;
    for _ in 3..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

fn main() {
    println!("{}", climb_stairs(3));
}

#[cfg(test)]
mod tests {
    use super::climb_stairs;

    #[test]
    fn example_one() {
        assert_eq!(climb_stairs(2), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(climb_stairs(3), 3);
    }
}
