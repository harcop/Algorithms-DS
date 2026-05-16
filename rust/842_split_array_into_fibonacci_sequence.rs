/// LeetCode #842 - Split Array into Fibonacci Sequence
fn split_into_fibonacci(num: String) -> Vec<i32> {
    let b = num.as_bytes();
    let mut path = Vec::new();
    let mut out = None;

    fn dfs(b: &[u8], start: usize, path: &mut Vec<i32>, out: &mut Option<Vec<i32>>) -> bool {
        if start == b.len() {
            if path.len() >= 3 {
                *out = Some(path.clone());
                return true;
            }
            return false;
        }
        let mut val: i64 = 0;
        for i in start..b.len() {
            if i > start && b[start] == b'0' {
                break;
            }
            val = val * 10 + (b[i] - b'0') as i64;
            if val > i32::MAX as i64 {
                break;
            }
            let v = val as i32;
            if path.len() >= 2 && v != path[path.len() - 1] + path[path.len() - 2] {
                continue;
            }
            path.push(v);
            if dfs(b, i + 1, path, out) {
                return true;
            }
            path.pop();
            if val == 0 {
                break;
            }
        }
        false
    }

    dfs(b, 0, &mut path, &mut out);
    out.unwrap_or_default()
}

fn main() {
    println!("{:?}", split_into_fibonacci("1101111".into()));
}

#[cfg(test)]
mod tests {
    use super::split_into_fibonacci;

    #[test]
    fn example_one() {
        assert_eq!(split_into_fibonacci("1101111".into()), vec![11, 0, 11, 11]);
    }
}
