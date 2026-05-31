/// LeetCode #1614 - Maximum Nesting Depth Of The Parentheses
fn max_depth(s: String) -> i32 {
    let mut d = 0i32;
    let mut ans = 0i32;
    for c in s.bytes() {
        if c == b'(' { d += 1; ans = ans.max(d); }
        else if c == b')' { d -= 1; }
    }
    ans
}
fn main() { println!("{}", max_depth("(1+(2*3)+((8)/4))+1".into())); }
#[cfg(test)]
mod tests {
    use super::max_depth;
    #[test]
    fn example_one() { assert_eq!(max_depth("(1+(2*3)+((8)/4))+1".into()), 3); }
}