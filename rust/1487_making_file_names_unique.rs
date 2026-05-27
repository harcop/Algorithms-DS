/// LeetCode #1487 - Making File Names Unique
use std::collections::HashMap;
fn get_folder_names(names: Vec<String>) -> Vec<String> {
    let mut used = HashMap::new();
    let mut res = Vec::new();
    for name in names {
        if !used.contains_key(&name) {
            used.insert(name.clone(), 1);
            res.push(name);
            continue;
        }
        let mut k = used[&name];
        let mut candidate = format!("{}({})", name, k);
        while used.contains_key(&candidate) {
            k += 1;
            candidate = format!("{}({})", name, k);
        }
        used.insert(name.clone(), k + 1);
        used.insert(candidate.clone(), 1);
        res.push(candidate);
    }
    res
}
fn main() { println!("{:?}", get_folder_names(vec!["pes".into(),"foder".into(),"g".into()])); }
#[cfg(test)]
mod tests {
    use super::get_folder_names;
    #[test]
    fn example_one() {
        assert_eq!(get_folder_names(vec!["pes".into(),"foder".into(),"g".into()]), vec!["pes".to_string(),"foder".to_string(),"g".to_string()]);
    }
    #[test]
    fn example_two() {
        assert_eq!(get_folder_names(vec!["g".into(),"k".into(),"o".into(),"g".into()]), vec!["g".to_string(),"k".to_string(),"o".to_string(),"g(1)".to_string()]);
    }
}