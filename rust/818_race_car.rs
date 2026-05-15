/// LeetCode #818 - Race Car
use std::collections::{HashSet, VecDeque};

fn racecar(target: i32) -> i32 {
    let mut q = VecDeque::new();
    q.push_back((0i32, 1i32, 0i32));
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    seen.insert((0, 1));
    while let Some((pos, speed, steps)) = q.pop_front() {
        if pos == target {
            return steps;
        }
        let accel_pos = pos + speed;
        let accel_speed = speed * 2;
        if accel_pos > 0 && accel_pos <= target + 100 && seen.insert((accel_pos, accel_speed)) {
            q.push_back((accel_pos, accel_speed, steps + 1));
        }
        let rev_pos = pos - 1;
        let rev_speed = if speed > 0 { -1 } else { 1 };
        if rev_pos >= 0 && rev_pos <= target + 100 && seen.insert((rev_pos, rev_speed)) {
            q.push_back((rev_pos, rev_speed, steps + 1));
        }
    }
    -1
}

fn main() {
    println!("{}", racecar(3));
}

#[cfg(test)]
mod tests {
    use super::racecar;

    #[test]
    fn example_one() {
        assert_eq!(racecar(3), 2);
    }

    #[test]
    fn example_two() {
        let steps = racecar(6);
        assert!(steps >= 4 && steps <= 6);
    }
}
