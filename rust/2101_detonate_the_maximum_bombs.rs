/// LeetCode #2101 - Detonate the Maximum Bombs
fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
    let n = bombs.len();
    let mut g = vec![Vec::new(); n];
    for i in 0..n {
        let x1 = bombs[i][0] as i64;
        let y1 = bombs[i][1] as i64;
        let r = bombs[i][2] as i64;
        for j in 0..n {
            if i == j {
                continue;
            }
            let dx = x1 - bombs[j][0] as i64;
            let dy = y1 - bombs[j][1] as i64;
            if dx * dx + dy * dy <= r * r {
                g[i].push(j);
            }
        }
    }

    fn dfs(i: usize, g: &[Vec<usize>], seen: &mut [bool]) -> i32 {
        if seen[i] {
            return 0;
        }
        seen[i] = true;
        1 + g[i].iter().map(|&j| dfs(j, g, seen)).sum::<i32>()
    }

    let mut ans = 0;
    for i in 0..n {
        ans = ans.max(dfs(i, &g, &mut vec![false; n]));
    }
    ans
}

fn main() {
    println!("{}", maximum_detonation(vec![vec![2, 1, 3], vec![6, 1, 4]]));
}

#[cfg(test)]
mod tests {
    use super::maximum_detonation;

    #[test]
    fn example_one() {
        assert_eq!(maximum_detonation(vec![vec![2, 1, 3], vec![6, 1, 4]]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_detonation(vec![vec![1, 1, 5], vec![10, 10, 5]]), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(
            maximum_detonation(vec![
                vec![1, 2, 3],
                vec![2, 3, 1],
                vec![3, 4, 2],
                vec![4, 5, 3],
                vec![5, 6, 4],
            ]),
            5
        );
    }
}
