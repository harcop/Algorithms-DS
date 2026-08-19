/// LeetCode #3302 - Find the Lexicographically Smallest Valid Sequence
fn valid_sequence(word1: String, word2: String) -> Vec<i32> {
    let a = word1.as_bytes();
    let b = word2.as_bytes();
    let m = a.len();
    let n = b.len();
    let mut suf = vec![0; m + 1];
    suf[m] = n as i32;
    let mut j = n as i32 - 1;
    for i in (0..m).rev() {
        if j >= 0 && a[i] == b[j as usize] {
            j -= 1;
        }
        suf[i] = j + 1;
    }
    let mut ans = Vec::new();
    let mut changed = false;
    j = 0;
    for (i, &c) in a.iter().enumerate() {
        if c == b[j as usize] || (!changed && suf[i + 1] <= j + 1) {
            if c != b[j as usize] {
                changed = true;
            }
            ans.push(i as i32);
            j += 1;
            if j == n as i32 {
                return ans;
            }
        }
    }
    vec![]
}

fn main() {
    println!("{:?}", valid_sequence("vbcca".into(), "abc".into()));
}

#[cfg(test)]
mod tests {
    use super::valid_sequence;

    #[test]
    fn example1() {
        assert_eq!(valid_sequence("vbcca".into(), "abc".into()), vec![0, 1, 2]);
    }

    #[test]
    fn example2() {
        assert_eq!(valid_sequence("bacdc".into(), "abc".into()), vec![1, 2, 4]);
    }

    #[test]
    fn example3() {
        assert_eq!(
            valid_sequence("aaaaaa".into(), "aaabc".into()),
            vec![] as Vec<i32>
        );
    }

    #[test]
    fn example4() {
        assert_eq!(valid_sequence("abc".into(), "ab".into()), vec![0, 1]);
    }
}
