/// LeetCode #77 - Combinations
fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut out = Vec::new();
    let mut path = Vec::with_capacity(k as usize);
    backtrack(n, k, 1, &mut path, &mut out);
    out
}

fn backtrack(n: i32, k: i32, start: i32, path: &mut Vec<i32>, out: &mut Vec<Vec<i32>>) {
    if path.len() == k as usize {
        out.push(path.clone());
        return;
    }
    for i in start..=n {
        path.push(i);
        backtrack(n, k, i + 1, path, out);
        path.pop();
    }
}

fn main() {
    println!("{:?}", combine(4, 2));
}

#[cfg(test)]
mod tests {
    use super::combine;

    fn normalize(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        v.sort_unstable();
        v
    }

    #[test]
    fn example_one() {
        let got = normalize(combine(4, 2));
        let expected = normalize(vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4],
        ]);
        assert_eq!(got, expected);
    }

    #[test]
    fn example_two() {
        assert_eq!(combine(1, 1), vec![vec![1]]);
    }
}
