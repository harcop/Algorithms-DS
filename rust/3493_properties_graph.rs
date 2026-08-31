/// LeetCode #3493 - Properties Graph
fn number_of_components(properties: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = properties.len();
    let sets: Vec<std::collections::HashSet<i32>> = properties
        .iter()
        .map(|p| p.iter().copied().collect())
        .collect();
    let mut g = vec![Vec::new(); n];
    for i in 0..n {
        for j in 0..i {
            if sets[i].intersection(&sets[j]).count() >= k as usize {
                g[i].push(j);
                g[j].push(i);
            }
        }
    }
    let mut vis = vec![false; n];
    fn dfs(i: usize, g: &[Vec<usize>], vis: &mut [bool]) {
        vis[i] = true;
        for &j in &g[i] {
            if !vis[j] {
                dfs(j, g, vis);
            }
        }
    }
    let mut ans = 0;
    for i in 0..n {
        if !vis[i] {
            dfs(i, &g, &mut vis);
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        number_of_components(vec![vec![1, 2], vec![1, 1], vec![3, 4], vec![4, 5], vec![5, 6], vec![7, 7]], 1)
    );
}

#[cfg(test)]
mod tests {
    use super::number_of_components;

    #[test]
    fn example1() {
        assert_eq!(
            number_of_components(
                vec![vec![1, 2], vec![1, 1], vec![3, 4], vec![4, 5], vec![5, 6], vec![7, 7]],
                1
            ),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            number_of_components(vec![vec![1, 2, 3], vec![2, 3, 4], vec![4, 3, 5]], 2),
            1
        );
    }

    #[test]
    fn example3() {
        assert_eq!(number_of_components(vec![vec![1, 1], vec![1, 1]], 2), 2);
    }
}
