/// LeetCode #3662 - Filter Characters by Frequency (premium)
fn filter_characters(s: String, k: i32) -> String {
    let mut cnt = [0i32; 26];
    for b in s.bytes() {
        cnt[(b - b'a') as usize] += 1;
    }
    let mut ans = String::new();
    for b in s.bytes() {
        if cnt[(b - b'a') as usize] < k {
            ans.push(b as char);
        }
    }
    ans
}

fn main() {
    println!("{}", filter_characters("aadbbcccca".into(), 3));
}

#[cfg(test)]
mod tests {
    use super::filter_characters;

    #[test]
    fn example1() {
        assert_eq!(filter_characters("aadbbcccca".into(), 3), "dbb");
    }

    #[test]
    fn example2() {
        assert_eq!(filter_characters("xyz".into(), 2), "xyz");
    }
}
