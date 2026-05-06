/// LeetCode #365 - Water and Jug Problem
fn can_measure_water(x: i32, y: i32, target: i32) -> bool {
    if target < 0 {
        return false;
    }
    if target == 0 {
        return true;
    }
    let a = x as i64;
    let b = y as i64;
    let t = target as i64;
    if t > a + b {
        return false;
    }
    if a == 0 {
        return t == b;
    }
    if b == 0 {
        return t == a;
    }
    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a.abs()
    }
    t % gcd(a, b) == 0
}

fn main() {
    println!("{}", can_measure_water(3, 5, 4));
}

#[cfg(test)]
mod tests {
    use super::can_measure_water;

    #[test]
    fn example_one() {
        assert!(can_measure_water(3, 5, 4));
    }

    #[test]
    fn example_two() {
        assert!(!can_measure_water(2, 6, 5));
    }
}
