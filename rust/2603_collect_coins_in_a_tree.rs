/// LeetCode #2603 - Collect Coins in a Tree
use std::collections::{HashSet, VecDeque};

fn collect_the_coins(coins: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
    let n = coins.len();
    let mut g: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for e in &edges {
        let a = e[0] as usize;
        let b = e[1] as usize;
        g[a].insert(b);
        g[b].insert(a);
    }

    let mut q = VecDeque::new();
    for i in 0..n {
        if coins[i] == 0 && g[i].len() == 1 {
            q.push_back(i);
        }
    }
    while let Some(i) = q.pop_front() {
        let neighbors: Vec<usize> = g[i].iter().copied().collect();
        for j in neighbors {
            g[j].remove(&i);
            if coins[j] == 0 && g[j].len() == 1 {
                q.push_back(j);
            }
        }
        g[i].clear();
    }

    for _ in 0..2 {
        let leaves: Vec<usize> = (0..n).filter(|&i| g[i].len() == 1).collect();
        for i in leaves {
            let neighbors: Vec<usize> = g[i].iter().copied().collect();
            for j in neighbors {
                g[j].remove(&i);
            }
            g[i].clear();
        }
    }

    let mut ans = 0;
    for e in &edges {
        let a = e[0] as usize;
        let b = e[1] as usize;
        if !g[a].is_empty() && !g[b].is_empty() {
            ans += 2;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        collect_the_coins(
            vec![1, 0, 0, 0, 0, 1],
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::collect_the_coins;

    #[test]
    fn example_one() {
        assert_eq!(
            collect_the_coins(
                vec![1, 0, 0, 0, 0, 1],
                vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]
            ),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            collect_the_coins(
                vec![0, 0, 0, 1, 1, 0, 0, 1],
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![1, 3],
                    vec![1, 4],
                    vec![2, 5],
                    vec![5, 6],
                    vec![5, 7]
                ]
            ),
            2
        );
    }
}
