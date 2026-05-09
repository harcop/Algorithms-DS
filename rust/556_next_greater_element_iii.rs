/// LeetCode #556 - Next Greater Element III
fn next_greater_element(n: i32) -> i32 {
    let mut digits: Vec<u8> = n.to_string().bytes().collect();
    let mut i = digits.len().saturating_sub(2) as isize;
    while i >= 0 && digits[i as usize] >= digits[i as usize + 1] {
        i -= 1;
    }
    if i < 0 {
        return -1;
    }
    let i = i as usize;
    let mut j = digits.len() - 1;
    while digits[j] <= digits[i] {
        j -= 1;
    }
    digits.swap(i, j);
    digits[i + 1..].reverse();
    let s = String::from_utf8(digits).unwrap();
    match s.parse::<i32>() {
        Ok(x) if x > n => x,
        _ => -1,
    }
}

fn main() {
    println!("{}", next_greater_element(12));
}

#[cfg(test)]
mod tests {
    use super::next_greater_element;

    #[test]
    fn example_one() {
        assert_eq!(next_greater_element(12), 21);
    }
}
