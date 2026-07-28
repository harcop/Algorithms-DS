/// LeetCode #2729 - Check if The Number is Fascinating
fn is_fascinating(n: i32) -> bool {
    let s = format!("{}{}{}", n, n * 2, n * 3);
    let mut cnt = [0; 10];
    for c in s.chars() {
        let t = (c as usize) - ('0' as usize);
        cnt[t] += 1;
        if cnt[t] > 1 {
            return false;
        }
    }
    cnt[0] == 0 && s.len() == 9
}

fn main() {
    println!("{}", is_fascinating(192));
}

#[cfg(test)]
mod tests {
    use super::is_fascinating;

    #[test]
    fn example_one() {
        assert!(is_fascinating(192));
    }

    #[test]
    fn example_two() {
        assert!(!is_fascinating(100));
    }
}
