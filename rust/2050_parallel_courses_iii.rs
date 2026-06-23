/// LeetCode #2050 - Parallel Courses III
use std::collections::VecDeque;

fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
    let n = n as usize;
    let mut g = vec![Vec::new(); n];
    let mut indeg = vec![0i32; n];
    for r in relations {
        let a = (r[0] - 1) as usize;
        let b = (r[1] - 1) as usize;
        g[a].push(b);
        indeg[b] += 1;
    }

    let mut f = vec![0i32; n];
    let mut ans = 0i32;
    let mut q = VecDeque::new();
    for i in 0..n {
        if indeg[i] == 0 {
            f[i] = time[i];
            ans = ans.max(time[i]);
            q.push_back(i);
        }
    }

    while let Some(i) = q.pop_front() {
        for &j in &g[i] {
            f[j] = f[j].max(f[i] + time[j]);
            ans = ans.max(f[j]);
            indeg[j] -= 1;
            if indeg[j] == 0 {
                q.push_back(j);
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        minimum_time(
            3,
            vec![vec![1, 3], vec![2, 3]],
            vec![3, 2, 5],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_time;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_time(
                3,
                vec![vec![1, 3], vec![2, 3]],
                vec![3, 2, 5],
            ),
            8
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            minimum_time(
                5,
                vec![vec![1, 5], vec![2, 5], vec![3, 5], vec![3, 4], vec![4, 5]],
                vec![1, 2, 3, 4, 5],
            ),
            12
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_time(2, vec![], vec![1, 2]), 2);
    }
}
