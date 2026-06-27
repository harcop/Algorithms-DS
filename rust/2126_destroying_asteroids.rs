/// LeetCode #2126 - Destroying Asteroids
fn asteroids_destroyed(mass: i32, mut asteroids: Vec<i32>) -> bool {
    asteroids.sort_unstable();
    let mut mass = mass as i64;

    for asteroid in asteroids {
        if mass < asteroid as i64 {
            return false;
        }
        mass += asteroid as i64;
    }

    true
}

fn main() {
    println!("{}", asteroids_destroyed(10, vec![3, 9, 19, 5, 21]));
}

#[cfg(test)]
mod tests {
    use super::asteroids_destroyed;

    #[test]
    fn example_one() {
        assert!(asteroids_destroyed(10, vec![3, 9, 19, 5, 21]));
    }

    #[test]
    fn example_two() {
        assert!(!asteroids_destroyed(5, vec![4, 9, 23, 4]));
    }
}
