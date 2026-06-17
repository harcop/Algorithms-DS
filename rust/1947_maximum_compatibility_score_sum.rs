/// LeetCode #1947 - Maximum Compatibility Score Sum
fn max_compatibility_sum(students: Vec<Vec<i32>>, mentors: Vec<Vec<i32>>) -> i32 {
    let m = students.len();
    let mut g = vec![vec![0i32; m]; m];
    for i in 0..m {
        for j in 0..m {
            g[i][j] = students[i]
                .iter()
                .zip(mentors[j].iter())
                .filter(|(&a, &b)| a == b)
                .count() as i32;
        }
    }

    let mut vis = vec![false; m];
    let mut ans = 0i32;

    fn dfs(i: usize, s: i32, m: usize, g: &[Vec<i32>], vis: &mut [bool], ans: &mut i32) {
        if i >= m {
            *ans = (*ans).max(s);
            return;
        }
        for j in 0..m {
            if !vis[j] {
                vis[j] = true;
                dfs(i + 1, s + g[i][j], m, g, vis, ans);
                vis[j] = false;
            }
        }
    }

    dfs(0, 0, m, &g, &mut vis, &mut ans);
    ans
}

fn main() {
    println!(
        "{}",
        max_compatibility_sum(
            vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 1]],
            vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 1, 0]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::max_compatibility_sum;

    #[test]
    fn example_one() {
        assert_eq!(
            max_compatibility_sum(
                vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 1]],
                vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 1, 0]]
            ),
            8
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            max_compatibility_sum(
                vec![vec![0, 0], vec![0, 0], vec![0, 0]],
                vec![vec![1, 1], vec![1, 1], vec![1, 1]]
            ),
            0
        );
    }
}
