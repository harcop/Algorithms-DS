/// LeetCode #282 - Expression Add Operators
fn add_operators(num: String, target: i64) -> Vec<String> {
    let b = num.as_bytes();
    let mut out = vec![];
    fn dfs(
        b: &[u8],
        num: &str,
        i: usize,
        path: &mut String,
        cur: i64,
        prev: i64,
        target: i64,
        out: &mut Vec<String>,
    ) {
        if i == b.len() {
            if cur == target {
                out.push(path.clone());
            }
            return;
        }
        for j in i..b.len() {
            if j > i && b[i] == b'0' {
                break;
            }
            let slice = &num[i..=j];
            let val: i64 = slice.parse().unwrap();
            let len = path.len();
            if i == 0 {
                path.push_str(slice);
                dfs(b, num, j + 1, path, cur + val, val, target, out);
                path.truncate(len);
            } else {
                path.push('+');
                path.push_str(slice);
                dfs(b, num, j + 1, path, cur + val, val, target, out);
                path.truncate(len);

                path.push('-');
                path.push_str(slice);
                dfs(b, num, j + 1, path, cur - val, -val, target, out);
                path.truncate(len);

                path.push('*');
                path.push_str(slice);
                dfs(b, num, j + 1, path, cur - prev + prev * val, prev * val, target, out);
                path.truncate(len);
            }
        }
    }
    let mut path = String::new();
    dfs(b, &num, 0, &mut path, 0, 0, target, &mut out);
    out
}

fn main() {
    println!("{:?}", add_operators("123".into(), 6));
}

#[cfg(test)]
mod tests {
    use super::add_operators;

    #[test]
    fn example_one() {
        let mut v = add_operators("123".into(), 6);
        v.sort();
        assert_eq!(v, vec!["1*2*3".to_string(), "1+2+3".to_string()]);
    }
}
