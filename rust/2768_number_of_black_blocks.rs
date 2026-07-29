/// LeetCode #2768 - Number of Black Blocks
use std::collections::HashMap;

fn count_black_blocks(m: i64, n: i64, coordinates: Vec<Vec<i32>>) -> Vec<i64> {
    let dirs = [0i64, 0, -1, -1, 0];
    let mut cnt: HashMap<i64, i32> = HashMap::new();
    for c in &coordinates {
        let (x, y) = (c[0] as i64, c[1] as i64);
        for k in 0..4 {
            let (i, j) = (x + dirs[k], y + dirs[k + 1]);
            if i >= 0 && i < m - 1 && j >= 0 && j < n - 1 {
                *cnt.entry(i * n + j).or_insert(0) += 1;
            }
        }
    }
    let mut ans = vec![0i64; 5];
    ans[0] = (m - 1) * (n - 1);
    for &v in cnt.values() {
        ans[v as usize] += 1;
        ans[0] -= 1;
    }
    ans
}

fn main() {
    println!("{:?}", count_black_blocks(3, 3, vec![vec![0, 0]]));
}

#[cfg(test)]
mod tests {
    use super::count_black_blocks;

    #[test]
    fn example_one() {
        assert_eq!(
            count_black_blocks(3, 3, vec![vec![0, 0]]),
            vec![3, 1, 0, 0, 0]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            count_black_blocks(3, 3, vec![vec![0, 0], vec![1, 1], vec![0, 2]]),
            vec![0, 2, 2, 0, 0]
        );
    }
}
