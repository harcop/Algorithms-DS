/// LeetCode #402 - Remove K Digits
fn remove_kdigits(num: String, k: i32) -> String {
    let mut k = k as i32;
    let mut st: Vec<u8> = vec![];
    for b in num.bytes() {
        while k > 0 && !st.is_empty() && *st.last().unwrap() > b {
            st.pop();
            k -= 1;
        }
        st.push(b);
    }
    while k > 0 && !st.is_empty() {
        st.pop();
        k -= 1;
    }
    let mut i = 0usize;
    while i < st.len() && st[i] == b'0' {
        i += 1;
    }
    let s = String::from_utf8(st[i..].to_vec()).unwrap();
    if s.is_empty() { "0".into() } else { s }
}

fn main() {
    println!("{}", remove_kdigits("1432219".into(), 3));
}

#[cfg(test)]
mod tests {
    use super::remove_kdigits;

    #[test]
    fn example_one() {
        assert_eq!(remove_kdigits("1432219".into(), 3), "1219");
    }
}
