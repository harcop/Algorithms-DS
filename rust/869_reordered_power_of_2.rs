/// LeetCode #869 - Reordered Power of 2
fn reordered_power_of2(n: i32) -> bool {
    fn sig(mut x: i32) -> Vec<u8> {
        let mut cnt = [0u8; 10];
        while x > 0 {
            cnt[(x % 10) as usize] += 1;
            x /= 10;
        }
        cnt.to_vec()
    }
    let s = sig(n);
    for k in 0..31 {
        if sig(1 << k) == s {
            return true;
        }
    }
    false
}

fn main() {
    println!("{}", reordered_power_of2(1));
}

#[cfg(test)]
mod tests {
    use super::reordered_power_of2;

    #[test]
    fn example_one() {
        assert!(reordered_power_of2(1));
    }

    #[test]
    fn example_two() {
        assert!(!reordered_power_of2(10));
    }
}
