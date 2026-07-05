/// LeetCode #2231 - Largest Number After Digit Swaps by Parity
use std::collections::HashMap;

fn largest_integer(num: i32) -> i32 {
    let digits: Vec<i32> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    let mut cnt = HashMap::new();
    for &d in &digits {
        *cnt.entry(d).or_insert(0) += 1;
    }

    let mut idx = [8i32, 9];
    let mut ans = 0i32;
    for &x in &digits {
        let parity = (x & 1) as usize;
        while *cnt.get(&idx[parity]).unwrap_or(&0) == 0 {
            idx[parity] -= 2;
        }
        ans = ans * 10 + idx[parity];
        *cnt.get_mut(&idx[parity]).unwrap() -= 1;
    }
    ans
}

fn main() {
    println!("{}", largest_integer(1234));
}

#[cfg(test)]
mod tests {
    use super::largest_integer;

    #[test]
    fn example_one() {
        assert_eq!(largest_integer(1234), 3412);
    }

    #[test]
    fn example_two() {
        assert_eq!(largest_integer(65875), 87655);
    }
}
