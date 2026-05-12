/// LeetCode #670 - Maximum Swap
fn maximum_swap(num: i32) -> i32 {
    let mut digits: Vec<u8> = num.to_string().into_bytes();
    let n = digits.len();
    let mut last = [usize::MAX; 10];
    for i in 0..n {
        last[(digits[i] - b'0') as usize] = i;
    }
    for i in 0..n {
        for d in (0..10).rev() {
            if d as u8 > digits[i] - b'0' && last[d] > i && last[d] != usize::MAX {
                let j = last[d];
                digits.swap(i, j);
                return std::str::from_utf8(&digits).unwrap().parse().unwrap();
            }
        }
    }
    num
}

fn main() {
    println!("{}", maximum_swap(2736));
}

#[cfg(test)]
mod tests {
    use super::maximum_swap;

    #[test]
    fn example_one() {
        assert_eq!(maximum_swap(2736), 7236);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_swap(9973), 9973);
    }
}
