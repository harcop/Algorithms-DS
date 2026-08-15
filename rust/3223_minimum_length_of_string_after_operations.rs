/// LeetCode #3223 - Minimum Length of String After Operations
fn minimum_length(s: String) -> i32 {
    let mut cnt = [0i32; 26];
    for b in s.bytes() {
        cnt[(b - b'a') as usize] += 1;
    }
    cnt.iter()
        .map(|&x| {
            if x == 0 {
                0
            } else if x % 2 == 1 {
                1
            } else {
                2
            }
        })
        .sum()
}

fn main() {
    println!("{}", minimum_length("abaacbcbb".into()));
}

#[cfg(test)]
mod tests {
    use super::minimum_length;

    #[test]
    fn example1() {
        assert_eq!(minimum_length("abaacbcbb".into()), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_length("aa".into()), 2);
    }
}
