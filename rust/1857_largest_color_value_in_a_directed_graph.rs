/// LeetCode #1857 - Largest Color Value in a Directed Graph
use std::collections::{HashMap, VecDeque};

fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
    let n = colors.len();
    let colors = colors.as_bytes();
    let mut indeg = vec![0i32; n];
    let mut g: HashMap<usize, Vec<usize>> = HashMap::new();
    for e in edges {
        let a = e[0] as usize;
        let b = e[1] as usize;
        g.entry(a).or_default().push(b);
        indeg[b] += 1;
    }

    let mut q = VecDeque::new();
    let mut dp = vec![[0i32; 26]; n];
    for i in 0..n {
        if indeg[i] == 0 {
            q.push_back(i);
            let c = (colors[i] - b'a') as usize;
            dp[i][c] = 1;
        }
    }

    let mut cnt = 0i32;
    let mut ans = 1i32;
    while let Some(i) = q.pop_front() {
        cnt += 1;
        if let Some(neighbors) = g.get(&i) {
            for &j in neighbors {
                indeg[j] -= 1;
                if indeg[j] == 0 {
                    q.push_back(j);
                }
                let c = (colors[j] - b'a') as usize;
                for k in 0..26 {
                    let v = dp[i][k] + if k == c { 1 } else { 0 };
                    if v > dp[j][k] {
                        dp[j][k] = v;
                        ans = ans.max(v);
                    }
                }
            }
        }
    }

    if cnt < n as i32 {
        -1
    } else {
        ans
    }
}

fn main() {
    println!(
        "{}",
        largest_path_value(
            "abaca".into(),
            vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![3, 4]],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::largest_path_value;

    #[test]
    fn example_one() {
        assert_eq!(
            largest_path_value(
                "abaca".into(),
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![3, 4]],
            ),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            largest_path_value("a".into(), vec![vec![0, 0]]),
            -1
        );
    }
}
