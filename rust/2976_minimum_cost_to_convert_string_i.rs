/// LeetCode #2976 - Minimum Cost to Convert String I
fn minimum_cost(
    source: String,
    target: String,
    original: Vec<char>,
    changed: Vec<char>,
    cost: Vec<i32>,
) -> i64 {
    const INF: i64 = 1_000_000_000_000;
    let mut g = [[INF; 26]; 26];
    for i in 0..26 {
        g[i][i] = 0;
    }
    for i in 0..original.len() {
        let x = (original[i] as u8 - b'a') as usize;
        let y = (changed[i] as u8 - b'a') as usize;
        g[x][y] = g[x][y].min(cost[i] as i64);
    }
    for k in 0..26 {
        for i in 0..26 {
            for j in 0..26 {
                if g[i][k] + g[k][j] < g[i][j] {
                    g[i][j] = g[i][k] + g[k][j];
                }
            }
        }
    }
    let mut ans = 0i64;
    for (a, b) in source.bytes().zip(target.bytes()) {
        if a != b {
            let x = (a - b'a') as usize;
            let y = (b - b'a') as usize;
            if g[x][y] >= INF {
                return -1;
            }
            ans += g[x][y];
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        minimum_cost(
            "abcd".into(),
            "acbe".into(),
            vec!['a', 'b', 'c', 'c', 'e', 'd'],
            vec!['b', 'c', 'b', 'e', 'b', 'e'],
            vec![2, 5, 5, 1, 2, 20]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_cost;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_cost(
                "abcd".into(),
                "acbe".into(),
                vec!['a', 'b', 'c', 'c', 'e', 'd'],
                vec!['b', 'c', 'b', 'e', 'b', 'e'],
                vec![2, 5, 5, 1, 2, 20]
            ),
            28
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            minimum_cost(
                "aaaa".into(),
                "bbbb".into(),
                vec!['a', 'c'],
                vec!['c', 'b'],
                vec![1, 2]
            ),
            12
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            minimum_cost(
                "abcd".into(),
                "abce".into(),
                vec!['a'],
                vec!['e'],
                vec![10000]
            ),
            -1
        );
    }
}
