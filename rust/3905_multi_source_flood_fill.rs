/// LeetCode #3905 - Multi Source Flood Fill
use std::collections::HashMap;

fn color_grid(n: i32, m: i32, sources: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let m = m as usize;
    let mut ans = vec![vec![0i32; m]; n];
    let mut q = Vec::new();
    for s in sources {
        let r = s[0] as usize;
        let c = s[1] as usize;
        ans[r][c] = s[2];
        q.push((r, c, s[2]));
    }
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    while !q.is_empty() {
        let mut vis: HashMap<(usize, usize), i32> = HashMap::new();
        for (r, c, color) in q.drain(..) {
            for (dr, dc) in dirs {
                let x = r as i32 + dr;
                let y = c as i32 + dc;
                if x < 0 || y < 0 || x >= n as i32 || y >= m as i32 {
                    continue;
                }
                let (x, y) = (x as usize, y as usize);
                if ans[x][y] != 0 {
                    continue;
                }
                let entry = vis.entry((x, y)).or_insert(0);
                *entry = (*entry).max(color);
            }
        }
        for ((x, y), color) in vis {
            ans[x][y] = color;
            q.push((x, y, color));
        }
    }
    ans
}

fn main() {
    println!("{:?}", color_grid(3, 3, vec![vec![0, 0, 1], vec![2, 2, 2]]));
}

#[cfg(test)]
mod tests {
    use super::color_grid;

    #[test]
    fn example1() {
        assert_eq!(
            color_grid(3, 3, vec![vec![0, 0, 1], vec![2, 2, 2]]),
            vec![vec![1, 1, 2], vec![1, 2, 2], vec![2, 2, 2]]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            color_grid(3, 3, vec![vec![0, 1, 3], vec![1, 1, 5]]),
            vec![vec![3, 3, 3], vec![5, 5, 5], vec![5, 5, 5]]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(color_grid(2, 2, vec![vec![1, 1, 5]]), vec![vec![5, 5], vec![5, 5]]);
    }
}
