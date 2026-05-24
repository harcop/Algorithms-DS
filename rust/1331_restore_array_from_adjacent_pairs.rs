/// LeetCode #1331 - Restore Array From Adjacent Pairs
use std::collections::HashMap;

fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
    let mut g: HashMap<i32, Vec<i32>> = HashMap::new();
    for p in adjacent_pairs {
        g.entry(p[0]).or_default().push(p[1]);
        g.entry(p[1]).or_default().push(p[0]);
    }
    let start = g.iter().filter(|(_, v)| v.len() == 1).map(|(&k, _)| k).min().unwrap();
    let mut ans = vec![start];
    let mut prev = i32::MIN;
    let mut cur = start;
    while ans.len() < g.len() {
        let next = g.get(&cur).unwrap().iter().copied().find(|&x| x != prev).unwrap();
        ans.push(next);
        prev = cur;
        cur = next;
    }
    ans
}

fn main() {
    println!("{:?}", restore_array(vec![vec![2, 1], vec![3, 4], vec![3, 2]]));
}

#[cfg(test)]
mod tests {
    use super::restore_array;

    #[test]
    fn example_one() {
        assert_eq!(restore_array(vec![vec![2, 1], vec![3, 4], vec![3, 2]]), vec![1, 2, 3, 4]);
    }

    #[test]
    fn example_two() {
        assert_eq!(restore_array(vec![vec![4, -2], vec![1, 4], vec![-3, 1]]), vec![-3, 1, 4, -2]);
    }
}
