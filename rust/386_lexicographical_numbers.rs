/// LeetCode #386 - Lexicographical Numbers
fn lexical_order(n: i32) -> Vec<i32> {
    let mut out = Vec::with_capacity(n as usize);
    fn dfs(cur: i32, n: i32, out: &mut Vec<i32>) {
        if cur > n { return; }
        out.push(cur);
        for d in 0..=9 { if cur*10+d<=n { dfs(cur*10+d, n, out); } }
    }
    for d in 1..=9 { dfs(d, n, &mut out); }
    out
}

fn main() { println!("{:?}", lexical_order(13).len()); }

#[cfg(test)] mod tests { use super::*; #[test] fn ex(){
    assert_eq!(lexical_order(13), vec![1,10,11,12,13,2,3,4,5,6,7,8,9]);
}}
