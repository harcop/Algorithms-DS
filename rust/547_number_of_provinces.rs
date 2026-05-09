/// LeetCode #547 - Number of Provinces
fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let n = is_connected.len();
    let mut seen = vec![false; n];
    let mut provinces = 0i32;
    fn dfs(i: usize, g: &[Vec<i32>], seen: &mut [bool]) {
        seen[i] = true;
        for j in 0..g.len() {
            if g[i][j] == 1 && !seen[j] {
                dfs(j, g, seen);
            }
        }
    }
    for i in 0..n {
        if !seen[i] {
            provinces += 1;
            dfs(i, &is_connected, &mut seen);
        }
    }
    provinces
}

fn main() {
    println!(
        "{}",
        find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::find_circle_num;

    #[test]
    fn example_one() {
        assert_eq!(
            find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]),
            2
        );
    }
}
