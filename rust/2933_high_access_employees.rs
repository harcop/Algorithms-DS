/// LeetCode #2933 - High-Access Employees
use std::collections::HashMap;

fn find_high_access_employees(access_times: Vec<Vec<String>>) -> Vec<String> {
    let mut d: HashMap<String, Vec<i32>> = HashMap::new();
    for entry in access_times {
        let name = entry[0].clone();
        let t = &entry[1];
        let minutes = t[..2].parse::<i32>().unwrap() * 60 + t[2..].parse::<i32>().unwrap();
        d.entry(name).or_default().push(minutes);
    }
    let mut ans = Vec::new();
    for (name, mut ts) in d {
        ts.sort_unstable();
        if (2..ts.len()).any(|i| ts[i] - ts[i - 2] < 60) {
            ans.push(name);
        }
    }
    ans
}

fn main() {
    let access = vec![
        vec!["a".into(), "0549".into()],
        vec!["b".into(), "0457".into()],
        vec!["a".into(), "0532".into()],
        vec!["a".into(), "0621".into()],
        vec!["b".into(), "0540".into()],
    ];
    println!("{:?}", find_high_access_employees(access));
}

#[cfg(test)]
mod tests {
    use super::find_high_access_employees;

    fn sorted(mut v: Vec<String>) -> Vec<String> {
        v.sort();
        v
    }

    #[test]
    fn example_one() {
        let access = vec![
            vec!["a".into(), "0549".into()],
            vec!["b".into(), "0457".into()],
            vec!["a".into(), "0532".into()],
            vec!["a".into(), "0621".into()],
            vec!["b".into(), "0540".into()],
        ];
        assert_eq!(
            sorted(find_high_access_employees(access)),
            vec![String::from("a")]
        );
    }

    #[test]
    fn example_two() {
        let access = vec![
            vec!["d".into(), "0002".into()],
            vec!["c".into(), "0808".into()],
            vec!["c".into(), "0829".into()],
            vec!["e".into(), "0215".into()],
            vec!["d".into(), "1508".into()],
            vec!["d".into(), "1444".into()],
            vec!["d".into(), "1410".into()],
            vec!["c".into(), "0809".into()],
        ];
        assert_eq!(
            sorted(find_high_access_employees(access)),
            vec![String::from("c"), String::from("d")]
        );
    }

    #[test]
    fn example_three() {
        let access = vec![
            vec!["cd".into(), "1025".into()],
            vec!["ab".into(), "1025".into()],
            vec!["cd".into(), "1046".into()],
            vec!["cd".into(), "1055".into()],
            vec!["ab".into(), "1124".into()],
            vec!["ab".into(), "1120".into()],
        ];
        assert_eq!(
            sorted(find_high_access_employees(access)),
            vec![String::from("ab"), String::from("cd")]
        );
    }
}
