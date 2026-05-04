/// LeetCode #271 - Encode and Decode Strings
fn encode(strs: Vec<String>) -> String {
    let mut out = String::new();
    for s in strs {
        out.push_str(&format!("{}:{}", s.len(), s));
    }
    out
}

fn decode(s: String) -> Vec<String> {
    let mut out = vec![];
    let b = s.as_bytes();
    let mut i = 0usize;
    while i < b.len() {
        let mut j = i;
        while j < b.len() && b[j] != b':' {
            j += 1;
        }
        let n: usize = std::str::from_utf8(&b[i..j]).unwrap().parse().unwrap();
        j += 1;
        out.push(String::from_utf8(b[j..j + n].to_vec()).unwrap());
        i = j + n;
    }
    out
}

fn main() {
    let v = vec!["lint".into(), "code".into(), "love".into(), "you".into()];
    println!("{:?}", decode(encode(v)));
}

#[cfg(test)]
mod tests {
    use super::{decode, encode};

    #[test]
    fn round_trip() {
        let v = vec!["lint".into(), "code".into(), "love".into(), "you".into()];
        assert_eq!(decode(encode(v.clone())), v);
    }
}
