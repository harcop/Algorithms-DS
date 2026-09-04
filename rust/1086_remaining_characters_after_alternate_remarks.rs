/// LeetCode #1086 - High Five
use std::collections::BTreeMap;

fn high_five(items: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut scores: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    for it in items {
        scores.entry(it[0]).or_default().push(it[1]);
    }
    let mut out = Vec::new();
    for (id, mut sc) in scores {
        sc.sort_unstable_by(|a, b| b.cmp(a));
        let avg = sc.iter().take(5).sum::<i32>() / 5;
        out.push(vec![id, avg]);
    }
    out
}

fn main() {
    let items = vec![
        vec![1, 91],
        vec![1, 92],
        vec![2, 93],
        vec![2, 97],
        vec![1, 60],
        vec![2, 77],
        vec![1, 65],
        vec![1, 87],
        vec![1, 100],
        vec![2, 100],
        vec![2, 76],
    ];
    println!("{:?}", high_five(items));
}

#[cfg(test)]
mod tests {
    use super::high_five;

    #[test]
    fn example_one() {
        let items = vec![
            vec![1, 91],
            vec![1, 92],
            vec![2, 93],
            vec![2, 97],
            vec![1, 60],
            vec![2, 77],
            vec![1, 65],
            vec![1, 87],
            vec![1, 100],
            vec![2, 100],
            vec![2, 76],
        ];
        assert_eq!(high_five(items), vec![vec![1, 87], vec![2, 88]]);
    }

    #[test]
    fn example_two() {
        let items = vec![
            vec![1, 100],
            vec![7, 100],
            vec![1, 100],
            vec![7, 100],
            vec![1, 100],
            vec![7, 100],
            vec![1, 100],
            vec![7, 100],
            vec![1, 100],
            vec![7, 100],
        ];
        assert_eq!(high_five(items), vec![vec![1, 100], vec![7, 100]]);
    }
}
