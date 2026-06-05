/// LeetCode #1743 - Restore the Array From Adjacent Pairs
use std::collections::HashMap;

fn walk(start: i32, g: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut ans = vec![start];
    let mut prev = i32::MIN;
    let mut cur = start;
    while ans.len() < g.len() {
        let nxt = *g[&cur].iter().find(|&&x| x != prev).unwrap();
        ans.push(nxt);
        prev = cur;
        cur = nxt;
    }
    ans
}

fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
    let mut g: HashMap<i32, Vec<i32>> = HashMap::new();
    for e in adjacent_pairs {
        g.entry(e[0]).or_default().push(e[1]);
        g.entry(e[1]).or_default().push(e[0]);
    }
    let mut starts: Vec<i32> = g.iter().filter(|(_, v)| v.len() == 1).map(|(&k, _)| k).collect();
    starts.sort_unstable();
    let mut best = walk(starts[0], &g);
    for &s in starts.iter().skip(1) {
        let cand = walk(s, &g);
        if cand < best {
            best = cand;
        }
    }
    best
}
fn main() {
    println!("{:?}", restore_array(vec![vec![2, 1], vec![3, 4], vec![3, 2]]));
}
#[cfg(test)]
mod tests {
    use super::restore_array;
    #[test]
    fn example_one() {
        assert_eq!(
            restore_array(vec![vec![2, 1], vec![3, 4], vec![3, 2]]),
            vec![1, 2, 3, 4]
        );
    }
    #[test]
    fn example_two() {
        let ans = restore_array(vec![vec![4, -2], vec![1, 4], vec![-3, 1]]);
        assert!(ans == vec![-2, 4, 1, -3] || ans == vec![-3, 1, 4, -2]);
    }
}
