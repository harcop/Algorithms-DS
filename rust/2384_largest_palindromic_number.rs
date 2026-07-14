/// LeetCode #2384 - Largest Palindromic Number
fn largest_palindromic(num: String) -> String {
    let mut cnt = [0; 10];
    for b in num.bytes() {
        cnt[(b - b'0') as usize] += 1;
    }

    let mut mid = String::new();
    for i in (0..=9).rev() {
        if cnt[i] % 2 == 1 {
            mid.push(char::from(b'0' + i as u8));
            cnt[i] -= 1;
            break;
        }
    }

    let mut half = String::new();
    for i in 0..=9 {
        cnt[i] /= 2;
        for _ in 0..cnt[i] {
            half.push(char::from(b'0' + i as u8));
        }
    }
    while half.ends_with('0') {
        half.pop();
    }

    let left: String = half.chars().rev().collect();
    let ans = format!("{left}{mid}{half}");
    if ans.is_empty() {
        "0".to_string()
    } else {
        ans
    }
}

fn main() {
    println!("{}", largest_palindromic("444947137".to_string()));
}

#[cfg(test)]
mod tests {
    use super::largest_palindromic;

    #[test]
    fn example_one() {
        assert_eq!(largest_palindromic("444947137".to_string()), "7449447");
    }

    #[test]
    fn example_two() {
        assert_eq!(largest_palindromic("00009".to_string()), "9");
    }
}
