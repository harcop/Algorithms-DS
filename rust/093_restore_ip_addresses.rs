/// LeetCode #93 - Restore IP Addresses
fn restore_ip_addresses(s: String) -> Vec<String> {
    let bytes = s.into_bytes();
    let mut out = Vec::new();
    let mut cur = Vec::new();
    backtrack(&bytes, 0, 0, &mut cur, &mut out);
    out
}

fn backtrack(
    bytes: &[u8],
    pos: usize,
    segment: usize,
    cur: &mut Vec<String>,
    out: &mut Vec<String>,
) {
    if segment == 4 {
        if pos == bytes.len() {
            out.push(cur.join("."));
        }
        return;
    }
    let mut val = 0i32;
    for i in pos..bytes.len().min(pos + 3) {
        val = val * 10 + (bytes[i] - b'0') as i32;
        if val > 255 {
            break;
        }
        if i != pos && bytes[pos] == b'0' {
            break;
        }
        cur.push(val.to_string());
        backtrack(bytes, i + 1, segment + 1, cur, out);
        cur.pop();
        if val == 0 {
            break;
        }
    }
}

fn main() {
    println!("{:?}", restore_ip_addresses("25525511135".to_string()));
}

#[cfg(test)]
mod tests {
    use super::restore_ip_addresses;

    #[test]
    fn example_one() {
        let mut got = restore_ip_addresses("25525511135".to_string());
        got.sort();
        let mut expected = vec![
            "255.255.11.135".to_string(),
            "255.255.111.35".to_string(),
        ];
        expected.sort();
        assert_eq!(got, expected);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            restore_ip_addresses("0000".to_string()),
            vec!["0.0.0.0".to_string()]
        );
    }
}
