/// LeetCode #991 - Broken Calculator
fn broken_calc(target: i32) -> i32 {
    let mut t = target as i64;
    let mut ops = 0i32;
    while t > 1 {
        if t % 2 == 0 {
            t /= 2;
        } else {
            t += 1;
        }
        ops += 1;
    }
    ops
}

fn main() {
    println!("{}", broken_calc(3));
}

#[cfg(test)]
mod tests {
    use super::broken_calc;

    #[test]
    fn example_one() {
        assert_eq!(broken_calc(2), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(broken_calc(3), 2);
    }
}
