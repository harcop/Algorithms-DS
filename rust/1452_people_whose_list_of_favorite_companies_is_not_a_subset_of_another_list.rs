/// LeetCode #1452 - People Whose List Of Favorite Companies Is Not A Subset Of Another List
use std::collections::HashSet;
fn idx_by_companies(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
    let sets: Vec<HashSet<String>> = favorite_companies.iter().map(|v| v.iter().cloned().collect()).collect();
    let mut res = Vec::new();
    for i in 0..sets.len() {
        let mut ok = true;
        for j in 0..sets.len() {
            if i != j && sets[i].is_subset(&sets[j]) { ok = false; break; }
        }
        if ok { res.push(i as i32); }
    }
    res
}
fn main() { println!("{:?}", idx_by_companies(vec![vec!["google".into(),"facebook".into()], vec!["google".into(),"microsoft".into()]])); }
#[cfg(test)]
mod tests {
    use super::idx_by_companies;
    #[test]
    fn example_one() {
        assert_eq!(idx_by_companies(vec![vec!["leetcode".into(),"google".into(),"facebook".into()], vec!["google".into(),"microsoft".into()]]), vec![0,1]);
    }
}