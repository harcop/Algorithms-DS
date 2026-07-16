/// LeetCode #2417 - Closest Fair Integer
fn is_fair(mut x: i32) -> bool {
    let mut even = 0;
    let mut odd = 0;
    while x > 0 {
        if (x % 10) % 2 == 0 {
            even += 1;
        } else {
            odd += 1;
        }
        x /= 10;
    }
    even == odd
}

fn closest_fair(n: i32) -> i32 {
    let digits = n.to_string().len();
    let mut x = if digits % 2 == 0 { n } else { 10_i32.pow(digits as u32) };
    loop {
        if is_fair(x) {
            return x;
        }
        x += 1;
    }
}

fn main() {
    println!("{}", closest_fair(403));
}

#[cfg(test)]
mod tests {
    use super::closest_fair;

    #[test]
    fn example_one() {
        assert_eq!(closest_fair(403), 1001);
    }

    #[test]
    fn example_two() {
        assert_eq!(closest_fair(10), 10);
    }
}
