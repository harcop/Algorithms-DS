/// LeetCode #591 - Tag Validator
fn is_valid(code: String) -> bool {
    fn valid_name(name: &str) -> bool {
        !name.is_empty()
            && name.len() <= 9
            && name.bytes().all(|b| b.is_ascii_uppercase())
    }
    fn valid_cdata(s: &str) -> bool {
        s.contains("CDATA[") && s.contains("]]")
    }
    let bytes = code.as_bytes();
    let n = bytes.len();
    if n < 2 || bytes[0] != b'<' {
        return false;
    }
    let mut i = 0usize;
    let mut stack: Vec<String> = vec![];
    while i < n {
        if i > 0 && stack.is_empty() {
            return false;
        }
        if bytes[i] == b'<' {
            if i + 1 < n && bytes[i + 1] == b'!' {
                if stack.is_empty() {
                    return false;
                }
                if i + 9 <= n && &bytes[i..i + 9] == b"<![CDATA[" {
                    let rest = String::from_utf8_lossy(&bytes[i..]);
                    if let Some(end) = rest.find("]]>") {
                        if !valid_cdata(&rest[..end + 3]) {
                            return false;
                        }
                        i += end + 3;
                        continue;
                    }
                }
                return false;
            }
            if i + 1 < n && bytes[i + 1] == b'/' {
                let j = i + 2;
                let k = match code[j..].find('>') {
                    Some(p) => j + p,
                    None => return false,
                };
                let tag = &code[j..k];
                if stack.last().map(|t| t.as_str()) != Some(tag) {
                    return false;
                }
                stack.pop();
                i = k + 1;
                continue;
            }
            let j = i + 1;
            let k = match code[j..].find('>') {
                Some(p) => j + p,
                None => return false,
            };
            let inner = &code[j..k];
            if inner.is_empty() || inner.contains('/') {
                return false;
            }
            let parts: Vec<&str> = inner.split_whitespace().collect();
            let tag = parts[0];
            if !valid_name(tag) {
                return false;
            }
            stack.push(tag.to_string());
            i = k + 1;
        } else {
            let j = match code[i..].find('<') {
                Some(p) => i + p,
                None => n,
            };
            let _content = &code[i..j];
            i = j;
        }
    }
    stack.is_empty()
}

fn main() {
    println!("{}", is_valid("<DIV>This is the first line <![CDATA[<div>]]></DIV>".into()));
}

#[cfg(test)]
mod tests {
    use super::is_valid;

    #[test]
    fn rejects_plain() {
        assert!(!is_valid("A<B>C</B>".into()));
    }
}
