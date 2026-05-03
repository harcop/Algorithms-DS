/// LeetCode #216 - Combination Sum III
fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    let mut out = vec![];
    let mut path = vec![];
    fn dfs(start: i32, k: i32, sum: i32, n: i32, path: &mut Vec<i32>, out: &mut Vec<Vec<i32>>) {
        if k == 0 && sum == n {
            out.push(path.clone());
            return;
        }
        if k == 0 || sum > n {
            return;
        }
        for x in start..=9 {
            if sum + x > n {
                break;
            }
            path.push(x);
            dfs(x + 1, k - 1, sum + x, n, path, out);
            path.pop();
        }
    }
    dfs(1, k, 0, n, &mut path, &mut out);
    out
}

fn main() {
    println!("{:?}", combination_sum3(3, 7));
}

#[cfg(test)]
mod tests {
    use super::combination_sum3;

    #[test]
    fn example_one() {
        let mut v = combination_sum3(3, 7);
        v.sort();
        assert_eq!(v, vec![vec![1, 2, 4]]);
    }

    #[test]
    fn example_two() {
        let mut v = combination_sum3(3, 9);
        v.sort();
        let mut e = vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]];
        e.sort();
        assert_eq!(v, e);
    }
}
