/// LeetCode #3669 - Balanced K-Factor Decomposition
fn min_difference(n: i32, k: i32) -> Vec<i32> {
    fn factors(x: i32) -> Vec<i32> {
        let mut v = Vec::new();
        let mut i = 1;
        while i * i <= x {
            if x % i == 0 {
                v.push(i);
                if i * i != x {
                    v.push(x / i);
                }
            }
            i += 1;
        }
        v
    }
    let mut ans = Vec::new();
    let mut path = vec![0; k as usize];
    let mut cur = i32::MAX;
    fn dfs(
        i: usize,
        x: i32,
        mi: i32,
        mx: i32,
        path: &mut [i32],
        cur: &mut i32,
        ans: &mut Vec<i32>,
    ) {
        if i == 0 {
            let d = mx.max(x) - mi.min(x);
            if d < *cur {
                *cur = d;
                path[0] = x;
                *ans = path.to_vec();
            }
            return;
        }
        for y in factors(x) {
            path[i] = y;
            dfs(i - 1, x / y, mi.min(y), mx.max(y), path, cur, ans);
        }
    }
    dfs(k as usize - 1, n, i32::MAX, 0, &mut path, &mut cur, &mut ans);
    ans
}

fn main() {
    println!("{:?}", min_difference(100, 2));
}

#[cfg(test)]
mod tests {
    use super::min_difference;

    fn product(v: &[i32]) -> i32 {
        v.iter().product()
    }

    fn spread(v: &[i32]) -> i32 {
        v.iter().max().unwrap() - v.iter().min().unwrap()
    }

    #[test]
    fn example1() {
        let ans = min_difference(100, 2);
        assert_eq!(ans.len(), 2);
        assert_eq!(product(&ans), 100);
        assert_eq!(spread(&ans), 0);
    }

    #[test]
    fn example2() {
        let ans = min_difference(44, 3);
        assert_eq!(ans.len(), 3);
        assert_eq!(product(&ans), 44);
        assert_eq!(spread(&ans), 9);
    }
}
