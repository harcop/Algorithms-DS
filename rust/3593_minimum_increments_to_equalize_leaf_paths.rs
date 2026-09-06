/// LeetCode #3593 - Minimum Increments to Equalize Leaf Paths
fn min_increase(n: i32, edges: Vec<Vec<i32>>, cost: Vec<i32>) -> i32 {
    let n = n as usize;
    let mut g = vec![Vec::new(); n];
    for e in &edges {
        let (a, b) = (e[0] as usize, e[1] as usize);
        g[a].push(b);
        g[b].push(a);
    }
    let mut ans = 0;
    fn dfs(u: usize, p: i32, g: &[Vec<usize>], cost: &[i32], ans: &mut i32) -> i64 {
        let mut mx = 0i64;
        let mut cnt = 0i32;
        let mut children = 0i32;
        for &v in &g[u] {
            if v as i32 == p {
                continue;
            }
            children += 1;
            let c = dfs(v, u as i32, g, cost, ans);
            if c > mx {
                mx = c;
                cnt = 1;
            } else if c == mx {
                cnt += 1;
            }
        }
        if children > 0 {
            *ans += children - cnt;
        }
        mx + cost[u] as i64
    }
    dfs(0, -1, &g, &cost, &mut ans);
    ans
}

fn main() {
    println!("{}", min_increase(3, vec![vec![0, 1], vec![0, 2]], vec![2, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::min_increase;

    #[test]
    fn example1() {
        assert_eq!(min_increase(3, vec![vec![0, 1], vec![0, 2]], vec![2, 1, 3]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_increase(3, vec![vec![0, 1], vec![1, 2]], vec![5, 1, 4]), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(
            min_increase(5, vec![vec![0, 4], vec![0, 1], vec![1, 2], vec![1, 3]], vec![3, 4, 1, 1, 7]),
            1
        );
    }
}
