/// LeetCode #2184 - Number of Ways to Build Sturdy Brick Wall
const MOD: i64 = 1_000_000_007;

fn build_wall(height: i32, width: i32, bricks: Vec<i32>) -> i32 {
    let height = height as usize;
    let width = width as usize;
    let bricks: Vec<usize> = bricks.into_iter().map(|x| x as usize).collect();

    let mut rows = Vec::new();
    let mut cur = Vec::new();

    fn dfs(
        width: usize,
        pos: usize,
        bricks: &[usize],
        cur: &mut Vec<usize>,
        rows: &mut Vec<Vec<usize>>,
    ) {
        if pos > width {
            return;
        }
        if pos == width {
            rows.push(cur.clone());
            return;
        }
        for &b in bricks {
            cur.push(b);
            dfs(width, pos + b, bricks, cur, rows);
            cur.pop();
        }
    }

    dfs(width, 0, &bricks, &mut cur, &mut rows);

    fn compatible(a: &[usize], b: &[usize]) -> bool {
        let mut i = 1usize;
        let mut j = 1usize;
        let mut s1 = a[0];
        let mut s2 = b[0];
        while i < a.len() && j < b.len() {
            if s1 == s2 {
                return false;
            }
            if s1 < s2 {
                s1 += a[i];
                i += 1;
            } else {
                s2 += b[j];
                j += 1;
            }
        }
        true
    }

    let n = rows.len();
    if n == 0 {
        return 0;
    }

    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        if compatible(&rows[i], &rows[i]) {
            graph[i].push(i);
        }
        for j in i + 1..n {
            if compatible(&rows[i], &rows[j]) {
                graph[i].push(j);
                graph[j].push(i);
            }
        }
    }

    let mut dp = vec![vec![0i64; n]; height];
    for j in 0..n {
        dp[0][j] = 1;
    }

    for i in 1..height {
        for j in 0..n {
            for &k in &graph[j] {
                dp[i][j] = (dp[i][j] + dp[i - 1][k]) % MOD;
            }
        }
    }

    dp[height - 1].iter().sum::<i64>() as i32 % MOD as i32
}

fn main() {
    println!("{}", build_wall(2, 3, vec![1, 2]));
}

#[cfg(test)]
mod tests {
    use super::build_wall;

    #[test]
    fn example_one() {
        assert_eq!(build_wall(2, 3, vec![1, 2]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(build_wall(1, 1, vec![5]), 0);
    }
}
