/// LeetCode #2813 - Maximum Elegance of a K-Length Subsequence
use std::collections::HashSet;

fn find_maximum_elegance(mut items: Vec<Vec<i32>>, k: usize) -> i64 {
    items.sort_by(|a, b| b[0].cmp(&a[0]));
    let mut tot = 0i64;
    let mut vis = HashSet::new();
    let mut dup = Vec::new();
    for item in items.iter().take(k) {
        let (p, c) = (item[0] as i64, item[1]);
        tot += p;
        if !vis.insert(c) {
            dup.push(p);
        }
    }
    let mut ans = tot + (vis.len() as i64).pow(2);
    for item in items.iter().skip(k) {
        let (p, c) = (item[0] as i64, item[1]);
        if vis.contains(&c) || dup.is_empty() {
            continue;
        }
        vis.insert(c);
        tot += p - dup.pop().unwrap();
        ans = ans.max(tot + (vis.len() as i64).pow(2));
    }
    ans
}

fn main() {
    println!("{}", find_maximum_elegance(vec![vec![3, 2], vec![5, 1], vec![10, 1]], 2));
}

#[cfg(test)]
mod tests {
    use super::find_maximum_elegance;

    #[test]
    fn example_one() {
        assert_eq!(
            find_maximum_elegance(vec![vec![3, 2], vec![5, 1], vec![10, 1]], 2),
            17
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_maximum_elegance(vec![vec![3, 1], vec![3, 1], vec![2, 2], vec![5, 3]], 3),
            19
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            find_maximum_elegance(vec![vec![1, 1], vec![2, 1], vec![3, 1]], 3),
            7
        );
    }
}
