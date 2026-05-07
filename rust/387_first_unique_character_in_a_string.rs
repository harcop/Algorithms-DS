/// LeetCode #387 - First Unique Character in a String
fn first_uniq_char(s: String) -> i32 {
    let mut c = [0i32; 256];
    for b in s.bytes() { c[b as usize]+=1; }
    for (i,b) in s.bytes().enumerate() { if c[b as usize]==1 { return i as i32; } }
    -1
}

fn main() { println!("{}", first_uniq_char("leetcode".into())); }

#[cfg(test)] mod tests { use super::*; #[test] fn ex(){
    assert_eq!(first_uniq_char("leetcode".into()),0);
}}
