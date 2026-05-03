/// LeetCode #202 - Happy Number
use std::collections::HashSet;

fn is_happy(n: i32) -> bool {
    let mut seen = HashSet::new();
    let mut x = n;
    loop {
        if x == 1 {
            return true;
        }
        if !seen.insert(x) {
            return false;
        }
        let mut s = 0;
        let mut t = x;
        while t > 0 {
            let d = t % 10;
            s += d * d;
            t /= 10;
        }
        x = s;
    }
}

fn main() {
    println!("{}", is_happy(19));
}

#[cfg(test)]
mod tests {
    use super::is_happy;

    #[test]
    fn example_one() {
        assert!(is_happy(19));
    }

    #[test]
    fn example_two() {
        assert!(!is_happy(2));
    }
}
