/// LeetCode #1317 - Convert Integer to the Sum of Two No Zero Integers
fn no_zero_integers(n: i32) -> Vec<i32> {
    for a in 1..n {
        let b = n - a;
        if !has_zero(a) && !has_zero(b) {
            return vec![a, b];
        }
    }
    vec![]
}

fn has_zero(x: i32) -> bool {
    let mut x = x;
    while x > 0 {
        if x % 10 == 0 {
            return true;
        }
        x /= 10;
    }
    false
}

fn main() {
    println!("{:?}", no_zero_integers(11));
}

#[cfg(test)]
mod tests {
    use super::no_zero_integers;

    #[test]
    fn example_one() {
        assert_eq!(no_zero_integers(2), vec![1, 1]);
    }

    #[test]
    fn example_two() {
        assert_eq!(no_zero_integers(11), vec![2, 9]);
    }
}
