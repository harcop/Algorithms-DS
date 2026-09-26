/// LeetCode #3955 - Valid Binary Strings With Cost Limit
fn generate_valid_strings(n: i32, k: i32) -> Vec<String> {
    fn dfs(i: i32, tot: i32, n: i32, k: i32, path: &mut Vec<u8>, ans: &mut Vec<String>) {
        if i >= n {
            ans.push(String::from_utf8(path.clone()).unwrap());
            return;
        }
        path.push(b'0');
        dfs(i + 1, tot, n, k, path, ans);
        path.pop();
        if (path.is_empty() || *path.last().unwrap() == b'0') && tot + i <= k {
            path.push(b'1');
            dfs(i + 1, tot + i, n, k, path, ans);
            path.pop();
        }
    }
    let mut ans = Vec::new();
    let mut path = Vec::new();
    dfs(0, 0, n, k, &mut path, &mut ans);
    ans
}

fn main() {
    println!("{:?}", generate_valid_strings(3, 1));
}

#[cfg(test)]
mod tests {
    use super::generate_valid_strings;

    #[test]
    fn example1() {
        assert_eq!(
            generate_valid_strings(3, 1),
            vec!["000".to_string(), "010".to_string(), "100".to_string()]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            generate_valid_strings(1, 0),
            vec!["0".to_string(), "1".to_string()]
        );
    }
}
