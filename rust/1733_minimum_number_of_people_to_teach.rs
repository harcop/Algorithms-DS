/// LeetCode #1733 - Minimum Number of People to Teach
use std::collections::{HashMap, HashSet};

fn can_talk(languages: &[Vec<i32>], u: i32, v: i32) -> bool {
    let u = u as usize - 1;
    let v = v as usize - 1;
    languages[u].iter().any(|x| languages[v].contains(x))
}

fn minimum_teachings(_n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
    let mut need = HashSet::new();
    for f in friendships {
        let (u, v) = (f[0], f[1]);
        if !can_talk(&languages, u, v) {
            need.insert(u);
            need.insert(v);
        }
    }
    if need.is_empty() {
        return 0;
    }
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for u in &need {
        for &lang in &languages[*u as usize - 1] {
            *cnt.entry(lang).or_insert(0) += 1;
        }
    }
    let mx = cnt.values().copied().max().unwrap_or(0);
    need.len() as i32 - mx
}

fn main() {
    println!(
        "{}",
        minimum_teachings(
            2,
            vec![vec![1], vec![2], vec![1, 2]],
            vec![vec![1, 2], vec![1, 3], vec![2, 3]],
        )
    );
}
#[cfg(test)]
mod tests {
    use super::minimum_teachings;
    #[test]
    fn example_one() {
        assert_eq!(
            minimum_teachings(
                2,
                vec![vec![1], vec![2], vec![1, 2]],
                vec![vec![1, 2], vec![1, 3], vec![2, 3]],
            ),
            1
        );
    }
    #[test]
    fn example_two() {
        assert_eq!(
            minimum_teachings(
                3,
                vec![vec![2], vec![1, 3], vec![1, 2], vec![3]],
                vec![vec![1, 4], vec![1, 2], vec![3, 4], vec![2, 3]],
            ),
            2
        );
    }
}
