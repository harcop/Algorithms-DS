/// LeetCode #389 - Find the Difference (XOR)
fn find_the_difference(s: String, t: String) -> char {
    let mut x = 0u8;
    for b in s.bytes().chain(t.bytes()) { x ^= b; }
    x as char
}

fn main() { println!("{}", find_the_difference("abcd".into(), "abcde".into())); }

#[cfg(test)] mod tests { use super::*;
    #[test] fn smoke(){ assert_eq!(find_the_difference("".into(), "y".into()), 'y'); }
}
