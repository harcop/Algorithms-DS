/// LeetCode #2049 - Count Nodes With the Highest Score
fn count_highest_score_nodes(parents: Vec<i32>) -> i32 {
    let n = parents.len();
    let mut g = vec![Vec::new(); n];
    for i in 1..n {
        g[parents[i] as usize].push(i);
    }

    let mut ans = 0i32;
    let mut mx = 0i64;

    fn dfs(i: usize, fa: i32, g: &[Vec<usize>], n: usize, ans: &mut i32, mx: &mut i64) -> i32 {
        let mut cnt = 1i32;
        let mut score = 1i64;
        for &j in &g[i] {
            if j as i32 != fa {
                let t = dfs(j, i as i32, g, n, ans, mx);
                score *= t as i64;
                cnt += t;
            }
        }
        if n as i32 - cnt > 0 {
            score *= (n as i32 - cnt) as i64;
        }
        if *mx < score {
            *mx = score;
            *ans = 1;
        } else if *mx == score {
            *ans += 1;
        }
        cnt
    }

    dfs(0, -1, &g, n, &mut ans, &mut mx);
    ans
}

fn main() {
    println!("{}", count_highest_score_nodes(vec![-1, 2, 0, 2, 0]));
}

#[cfg(test)]
mod tests {
    use super::count_highest_score_nodes;

    #[test]
    fn example_one() {
        assert_eq!(count_highest_score_nodes(vec![-1, 2, 0, 2, 0]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_highest_score_nodes(vec![-1, 2, 0]), 2);
    }
}
