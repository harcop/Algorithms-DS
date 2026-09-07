/// LeetCode #3613 - Minimize Maximum Component Cost
fn find(p: &mut [i32], x: i32) -> i32 {
    let i = x as usize;
    if p[i] != x {
        p[i] = find(p, p[i]);
    }
    p[i]
}

fn min_cost(n: i32, mut edges: Vec<Vec<i32>>, k: i32) -> i32 {
    if k == n {
        return 0;
    }
    edges.sort_by_key(|e| e[2]);
    let mut p: Vec<i32> = (0..n).collect();
    let mut cnt = n;
    for e in edges {
        let pu = find(&mut p, e[0]);
        let pv = find(&mut p, e[1]);
        if pu != pv {
            p[pu as usize] = pv;
            cnt -= 1;
            if cnt <= k {
                return e[2];
            }
        }
    }
    0
}

fn main() {
    println!(
        "{}",
        min_cost(5, vec![vec![0, 1, 4], vec![1, 2, 3], vec![1, 3, 2], vec![3, 4, 6]], 2)
    );
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example1() {
        assert_eq!(
            min_cost(5, vec![vec![0, 1, 4], vec![1, 2, 3], vec![1, 3, 2], vec![3, 4, 6]], 2),
            4
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_cost(4, vec![vec![0, 1, 5], vec![1, 2, 5], vec![2, 3, 5]], 1),
            5
        );
    }
}
