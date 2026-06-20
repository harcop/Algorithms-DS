/// LeetCode #1999 - Smallest Greater Multiple Made of Two Digits
use std::collections::VecDeque;

fn find_integer(k: i32, digit1: i32, digit2: i32) -> i32 {
    if digit1 == 0 && digit2 == 0 {
        return -1;
    }
    let (d1, d2) = if digit1 > digit2 {
        (digit2, digit1)
    } else {
        (digit1, digit2)
    };
    let limit = i32::MAX as i64;
    let k = k as i64;
    let mut q = VecDeque::from([0i64]);
    while let Some(x) = q.pop_front() {
        if x > limit {
            return -1;
        }
        if x > k && x % k == 0 {
            return x as i32;
        }
        q.push_back(x * 10 + d1 as i64);
        if d1 != d2 {
            q.push_back(x * 10 + d2 as i64);
        }
    }
    -1
}

fn main() {
    println!("{}", find_integer(2, 0, 2));
}

#[cfg(test)]
mod tests {
    use super::find_integer;

    #[test]
    fn example_one() {
        assert_eq!(find_integer(2, 0, 2), 20);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_integer(3, 4, 2), 24);
    }

    #[test]
    fn example_three() {
        assert_eq!(find_integer(2, 0, 0), -1);
    }
}
