/// LeetCode #1604 - Alert Using Same Key Card Three Or More Times In A One Hour Period
use std::collections::HashMap;

fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
    let mut mp: HashMap<String, Vec<i32>> = HashMap::new();
    for (n, t) in key_name.into_iter().zip(key_time) {
        let p: Vec<i32> = t.split(':').map(|x| x.parse().unwrap()).collect();
        let mins = p[0] * 60 + p[1];
        mp.entry(n).or_default().push(mins);
    }
    let mut ans = vec![];
    for (name, mut times) in mp {
        times.sort_unstable();
        for i in 0..times.len().saturating_sub(2) {
            if times[i + 2] - times[i] <= 60 {
                ans.push(name);
                break;
            }
        }
    }
    ans.sort_unstable();
    ans
}
fn main() { println!("{:?}", alert_names(vec!["daniel".into()], vec!["10:00".into()])); }
#[cfg(test)]
mod tests {
    use super::alert_names;
    #[test]
    fn example_one() {
        assert_eq!(alert_names(vec!["daniel".into(),"daniel".into(),"daniel".into(),"luis".into(),"luis".into(),"luis".into(),"luis".into()], vec!["10:00".into(),"10:40".into(),"11:00".into(),"09:00".into(),"11:00".into(),"13:00".into(),"15:00".into()]), vec!["daniel"]);
    }
}