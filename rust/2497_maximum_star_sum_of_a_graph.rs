/// LeetCode #2497 - Maximum Star Sum of a Graph
fn max_star_sum(vals: Vec<i32>, edges: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = vals.len();
    let mut g = vec![Vec::new(); n];
    for e in edges {
        let a = e[0] as usize;
        let b = e[1] as usize;
        if vals[b] > 0 {
            g[a].push(vals[b]);
        }
        if vals[a] > 0 {
            g[b].push(vals[a]);
        }
    }
    for neighbors in &mut g {
        neighbors.sort_unstable_by(|a, b| b.cmp(a));
    }

    let k = k as usize;
    let mut ans = i32::MIN;
    for (i, &v) in vals.iter().enumerate() {
        let mut sum = v;
        for j in 0..k.min(g[i].len()) {
            sum += g[i][j];
        }
        ans = ans.max(sum);
    }
    ans
}

fn main() {
    println!(
        "{}",
        max_star_sum(
            vec![1, 2, 3, 4, 10, -10, -20],
            vec![
                vec![0, 1],
                vec![1, 2],
                vec![1, 3],
                vec![3, 4],
                vec![3, 5],
                vec![3, 6]
            ],
            2
        )
    );
}

#[cfg(test)]
mod tests {
    use super::max_star_sum;

    #[test]
    fn example_one() {
        assert_eq!(
            max_star_sum(
                vec![1, 2, 3, 4, 10, -10, -20],
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![1, 3],
                    vec![3, 4],
                    vec![3, 5],
                    vec![3, 6]
                ],
                2
            ),
            16
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(max_star_sum(vec![-5], vec![], 0), -5);
    }
}
