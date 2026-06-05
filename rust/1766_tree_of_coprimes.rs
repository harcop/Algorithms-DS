/// LeetCode #1766 - Tree of Coprimes
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a.abs()
}

fn get_coprimes(edges: Vec<Vec<i32>>, values: Vec<i32>) -> Vec<i32> {
    let n = values.len();
    let mut g = vec![vec![]; n];
    for e in edges {
        let u = (e[0] - 1) as usize;
        let v = (e[1] - 1) as usize;
        g[u].push(v);
        g[v].push(u);
    }
    let mut last = [-1i32; 51];
    let mut ans = vec![-1i32; n];

    enum Task {
        Enter(usize, usize, i32),
        Exit(usize, i32),
    }

    let mut stack = vec![Task::Enter(0, usize::MAX, 0)];
    while let Some(task) = stack.pop() {
        match task {
            Task::Enter(u, p, d) => {
                let v = values[u] as usize;
                let mut best = -1i32;
                for c in 1..=50 {
                    if gcd(c as i32, values[u]) == 1 && last[c] != -1 {
                        best = best.max(last[c]);
                    }
                }
                ans[u] = best;
                let old = last[v];
                last[v] = d;
                stack.push(Task::Exit(v, old));
                for &nb in g[u].iter().rev() {
                    if nb != p {
                        stack.push(Task::Enter(nb, u, d + 1));
                    }
                }
            }
            Task::Exit(v, old) => {
                last[v] = old;
            }
        }
    }
    ans
}
fn main() {
    println!(
        "{:?}",
        get_coprimes(
            vec![vec![2, 3], vec![6, 7], vec![3, 5], vec![1, 2], vec![7, 4], vec![1, 3]],
            vec![20, 6, 4, 3, 11, 14, 20, 10],
        )
    );
}
#[cfg(test)]
mod tests {
    use super::get_coprimes;
    #[test]
    fn example_one() {
        assert_eq!(
            get_coprimes(
                vec![vec![2, 3], vec![6, 7], vec![3, 5], vec![1, 2], vec![7, 4], vec![1, 3]],
                vec![20, 6, 4, 3, 11, 14, 20, 10],
            ),
            vec![-1, -1, -1, -1, -1, -1, 4, -1]
        );
    }
}
