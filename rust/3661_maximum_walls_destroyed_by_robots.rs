/// LeetCode #3661 - Maximum Walls Destroyed by Robots
fn lower_bound(a: &[i32], target: i32) -> usize {
    a.partition_point(|&x| x < target)
}

fn max_walls(robots: Vec<i32>, distance: Vec<i32>, mut walls: Vec<i32>) -> i32 {
    let n = robots.len();
    let mut arr: Vec<(i32, i32)> = robots.into_iter().zip(distance).collect();
    arr.sort_unstable_by_key(|p| p.0);
    walls.sort_unstable();
    let mut memo = vec![vec![-1; 2]; n];

    fn dfs(
        i: isize,
        j: usize,
        n: usize,
        arr: &[(i32, i32)],
        walls: &[i32],
        memo: &mut [Vec<i32>],
    ) -> i32 {
        if i < 0 {
            return 0;
        }
        let ui = i as usize;
        if memo[ui][j] != -1 {
            return memo[ui][j];
        }
        let mut left = arr[ui].0 - arr[ui].1;
        if ui > 0 {
            left = left.max(arr[ui - 1].0 + 1);
        }
        let l = lower_bound(walls, left);
        let r = lower_bound(walls, arr[ui].0 + 1);
        let mut ans = dfs(i - 1, 0, n, arr, walls, memo) + (r - l) as i32;

        let mut right = arr[ui].0 + arr[ui].1;
        if ui + 1 < n {
            if j == 0 {
                right = right.min(arr[ui + 1].0 - arr[ui + 1].1 - 1);
            } else {
                right = right.min(arr[ui + 1].0 - 1);
            }
        }
        let l = lower_bound(walls, arr[ui].0);
        let r = lower_bound(walls, right + 1);
        ans = ans.max(dfs(i - 1, 1, n, arr, walls, memo) + (r - l) as i32);
        memo[ui][j] = ans;
        ans
    }

    dfs(n as isize - 1, 1, n, &arr, &walls, &mut memo)
}

fn main() {
    println!("{}", max_walls(vec![4], vec![3], vec![1, 10]));
}

#[cfg(test)]
mod tests {
    use super::max_walls;

    #[test]
    fn example1() {
        assert_eq!(max_walls(vec![4], vec![3], vec![1, 10]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(max_walls(vec![10, 2], vec![5, 1], vec![5, 2, 7]), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(max_walls(vec![1, 2], vec![100, 1], vec![10]), 0);
    }
}
