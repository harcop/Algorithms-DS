/// LeetCode #423 - Reconstruct Original Digits from English
fn original_digits(s: String) -> String {
    let mut cnt = [0i32; 26];
    for b in s.bytes() {
        cnt[(b - b'a') as usize] += 1;
    }
    let mut d = [0i32; 10];
    d[0] = cnt['z' as usize - 'a' as usize];
    d[2] = cnt['w' as usize - 'a' as usize];
    d[4] = cnt['u' as usize - 'a' as usize];
    d[6] = cnt['x' as usize - 'a' as usize];
    d[8] = cnt['g' as usize - 'a' as usize];
    d[3] = cnt['h' as usize - 'a' as usize] - d[8];
    d[5] = cnt['f' as usize - 'a' as usize] - d[4];
    d[7] = cnt['s' as usize - 'a' as usize] - d[6];
    d[1] = cnt['o' as usize - 'a' as usize] - d[0] - d[2] - d[4];
    d[9] = cnt['i' as usize - 'a' as usize] - d[5] - d[6] - d[8];
    let mut out = String::new();
    for i in 0..10 {
        for _ in 0..d[i] {
            out.push((b'0' + i as u8) as char);
        }
    }
    out
}

fn main() {
    println!("{}", original_digits("owoztneoer".into()));
}

#[cfg(test)]
mod tests {
    use super::original_digits;

    #[test]
    fn example_one() {
        assert_eq!(original_digits("owoztneoer".into()), "012");
    }
}
