/// LeetCode #3849 - Maximum Bitwise XOR After Rearrangement
fn maximum_xor(s: String, t: String) -> String {
    let mut cnt = [0i32; 2];
    for c in t.bytes() {
        cnt[(c - b'0') as usize] += 1;
    }
    let mut ans = vec![b'0'; s.len()];
    for (i, c) in s.bytes().enumerate() {
        let x = (c - b'0') as usize;
        if cnt[x ^ 1] > 0 {
            cnt[x ^ 1] -= 1;
            ans[i] = b'1';
        } else {
            cnt[x] -= 1;
        }
    }
    String::from_utf8(ans).unwrap()
}

fn main() {
    println!("{}", maximum_xor("101".into(), "011".into()));
}

#[cfg(test)]
mod tests {
    use super::maximum_xor;

    #[test]
    fn example1() {
        assert_eq!(maximum_xor("101".into(), "011".into()), "110");
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_xor("0110".into(), "1110".into()), "1101");
    }

    #[test]
    fn example3() {
        assert_eq!(maximum_xor("0101".into(), "1001".into()), "1111");
    }
}
