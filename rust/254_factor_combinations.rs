/// LeetCode #254 - Factor Combinations
fn get_factors(n: i32) -> Vec<Vec<i32>> {
    let mut out = vec![];
    let mut path = vec![];
    fn dfs(start: i32, target: i32, path: &mut Vec<i32>, out: &mut Vec<Vec<i32>>) {
        if target > 1 && !path.is_empty() {
            let mut v = path.clone();
            v.push(target);
            out.push(v);
        }
        let lim = (target as f64).sqrt() as i32;
        for i in start..=lim {
            if target % i == 0 {
                path.push(i);
                dfs(i, target / i, path, out);
                path.pop();
            }
        }
    }
    dfs(2, n, &mut path, &mut out);
    out
}

fn main() {
    println!("{:?}", get_factors(1));
}

#[cfg(test)]
mod tests {
    use super::get_factors;

    #[test]
    fn example_one() {
        let mut v = get_factors(12);
        v.sort();
        let e = vec![vec![2, 6], vec![2, 2, 3], vec![3, 4]];
        assert_eq!(v.len(), e.len());
    }

    #[test]
    fn example_two() {
        assert!(get_factors(1).is_empty());
    }
}
