/// LeetCode #1598 - Crawler Log Folder
fn min_operations(logs: Vec<String>) -> i32 {
    let mut depth = 0i32;
    for log in logs {
        if log == "./" { continue; }
        if log == "../" { depth = depth.saturating_sub(1); }
        else { depth += 1; }
    }
    depth
}
fn main() { println!("{}", min_operations(vec!["d1/".into(),"d2/".into(),"../".into(),"d21/".into(),"./".into()])); }
#[cfg(test)]
mod tests {
    use super::min_operations;
    #[test]
    fn example_one() { assert_eq!(min_operations(vec!["d1/".into(),"d2/".into(),"../".into(),"d21/".into(),"./".into()]), 2); }
}