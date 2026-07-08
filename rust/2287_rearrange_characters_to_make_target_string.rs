/// LeetCode #2287 - Rearrange Characters to Make Target String
fn rearrange_characters(s: String, target: String) -> i32 {
    let mut fs = [0i32; 26];
    let mut ft = [0i32; 26];
    for b in s.bytes() {
        fs[(b - b'a') as usize] += 1;
    }
    for b in target.bytes() {
        ft[(b - b'a') as usize] += 1;
    }
    let mut ans = i32::MAX;
    for i in 0..26 {
        if ft[i] > 0 {
            ans = ans.min(fs[i] / ft[i]);
        }
    }
    if ans == i32::MAX { 0 } else { ans }
}

fn main() {
    println!(
        "{}",
        rearrange_characters("abcabc".to_string(), "abc".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::rearrange_characters;

    #[test]
    fn basic() {
        assert_eq!(
            rearrange_characters("abcabc".to_string(), "abc".to_string()),
            2
        );
    }

    #[test]
    fn insufficient() {
        assert_eq!(
            rearrange_characters("abbaccaddaeea".to_string(), "aaaaa".to_string()),
            1
        );
    }
}

