/// LeetCode #3385 - Minimum Time to Break Locks II
fn find_minimum_time(strength: Vec<i32>) -> i32 {
    let n = strength.len();
    let mut a = vec![vec![0i64; n]; n];
    for i in 0..n {
        for j in 0..n {
            let x = (j + 1) as i64;
            a[i][j] = (strength[i] as i64 + x - 1) / x;
        }
    }
    hungarian_min(&a) as i32
}

fn hungarian_min(a: &[Vec<i64>]) -> i64 {
    let n = a.len();
    let mut u = vec![0i64; n + 1];
    let mut v = vec![0i64; n + 1];
    let mut p = vec![0usize; n + 1];
    let mut way = vec![0usize; n + 1];
    for i in 1..=n {
        p[0] = i;
        let mut j0 = 0usize;
        let mut minv = vec![i64::MAX / 4; n + 1];
        let mut used = vec![false; n + 1];
        loop {
            used[j0] = true;
            let i0 = p[j0];
            let mut delta = i64::MAX / 4;
            let mut j1 = 0usize;
            for j in 1..=n {
                if !used[j] {
                    let cur = a[i0 - 1][j - 1] - u[i0] - v[j];
                    if cur < minv[j] {
                        minv[j] = cur;
                        way[j] = j0;
                    }
                    if minv[j] < delta {
                        delta = minv[j];
                        j1 = j;
                    }
                }
            }
            for j in 0..=n {
                if used[j] {
                    u[p[j]] += delta;
                    v[j] -= delta;
                } else {
                    minv[j] -= delta;
                }
            }
            j0 = j1;
            if p[j0] == 0 {
                break;
            }
        }
        loop {
            let j1 = way[j0];
            p[j0] = p[j1];
            j0 = j1;
            if j0 == 0 {
                break;
            }
        }
    }
    let mut ans = 0i64;
    for j in 1..=n {
        ans += a[p[j] - 1][j - 1];
    }
    ans
}

fn main() {
    println!("{}", find_minimum_time(vec![3, 4, 1]));
}

#[cfg(test)]
mod tests {
    use super::find_minimum_time;

    #[test]
    fn example1() {
        assert_eq!(find_minimum_time(vec![3, 4, 1]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(find_minimum_time(vec![2, 5, 4]), 6);
    }
}
