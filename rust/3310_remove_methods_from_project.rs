/// LeetCode #3310 - Remove Methods From Project
fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let k = k as usize;
    let mut g = vec![Vec::new(); n];
    for e in &invocations {
        g[e[0] as usize].push(e[1] as usize);
    }
    let mut suspicious = vec![false; n];
    let mut stack = vec![k];
    suspicious[k] = true;
    while let Some(i) = stack.pop() {
        for &j in &g[i] {
            if !suspicious[j] {
                suspicious[j] = true;
                stack.push(j);
            }
        }
    }
    for e in &invocations {
        let a = e[0] as usize;
        let b = e[1] as usize;
        if !suspicious[a] && suspicious[b] {
            return (0..n as i32).collect();
        }
    }
    (0..n as i32).filter(|&i| !suspicious[i as usize]).collect()
}

fn main() {
    println!(
        "{:?}",
        remaining_methods(4, 1, vec![vec![1, 2], vec![0, 1], vec![3, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::remaining_methods;

    #[test]
    fn example1() {
        let mut ans = remaining_methods(4, 1, vec![vec![1, 2], vec![0, 1], vec![3, 2]]);
        ans.sort_unstable();
        assert_eq!(ans, vec![0, 1, 2, 3]);
    }

    #[test]
    fn example2() {
        let mut ans = remaining_methods(
            5,
            0,
            vec![vec![1, 2], vec![0, 2], vec![0, 1], vec![3, 4]],
        );
        ans.sort_unstable();
        assert_eq!(ans, vec![3, 4]);
    }

    #[test]
    fn example3() {
        let ans = remaining_methods(3, 2, vec![vec![1, 2], vec![0, 1], vec![2, 0]]);
        assert_eq!(ans, vec![] as Vec<i32>);
    }
}
