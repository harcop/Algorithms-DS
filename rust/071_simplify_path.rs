/// LeetCode #71 - Simplify Path
fn simplify_path(path: String) -> String {
    let mut stack = Vec::new();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                stack.pop();
            }
            _ => stack.push(part),
        }
    }
    if stack.is_empty() {
        return "/".to_string();
    }
    let mut out = String::new();
    for s in stack {
        out.push('/');
        out.push_str(s);
    }
    out
}

fn main() {
    println!("{}", simplify_path("/home/".to_string()));
}

#[cfg(test)]
mod tests {
    use super::simplify_path;

    #[test]
    fn example_one() {
        assert_eq!(simplify_path("/home/".to_string()), "/home");
    }

    #[test]
    fn example_two() {
        assert_eq!(simplify_path("/home//foo/".to_string()), "/home/foo");
    }

    #[test]
    fn example_three() {
        assert_eq!(simplify_path("/home/user/Documents/../Pictures".to_string()), "/home/user/Pictures");
    }
}
