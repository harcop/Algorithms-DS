/// LeetCode #3044 - Most Frequent Prime
use std::collections::HashMap;

const DIRS: [(i32, i32); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn most_frequent_prime(mat: Vec<Vec<i32>>) -> i32 {
    if mat.is_empty() || mat[0].is_empty() {
        return -1;
    }
    let m = mat.len();
    let n = mat[0].len();
    let mut cnt: HashMap<i64, i32> = HashMap::new();

    for i in 0..m {
        for j in 0..n {
            for &(di, dj) in &DIRS {
                let mut x = i as i32;
                let mut y = j as i32;
                let mut v = 0i64;
                loop {
                    if x < 0 || x >= m as i32 || y < 0 || y >= n as i32 {
                        break;
                    }
                    v = v * 10 + mat[x as usize][y as usize] as i64;
                    if v >= 10 && is_prime(v) {
                        *cnt.entry(v).or_default() += 1;
                    }
                    x += di;
                    y += dj;
                }
            }
        }
    }

    let mut ans = -1i32;
    let mut max_cnt = 0;
    for (&prime, &c) in &cnt {
        if c > max_cnt || (c == max_cnt && prime > ans as i64) {
            max_cnt = c;
            ans = prime as i32;
        }
    }
    ans
}

fn main() {
    println!("{}", most_frequent_prime(vec![vec![1, 1], vec![9, 9], vec![1, 1]]));
}

#[cfg(test)]
mod tests {
    use super::most_frequent_prime;

    #[test]
    fn example1() {
        assert_eq!(
            most_frequent_prime(vec![vec![1, 1], vec![9, 9], vec![1, 1]]),
            19
        );
    }

    #[test]
    fn example2() {
        assert_eq!(most_frequent_prime(vec![vec![7]]), -1);
    }

    #[test]
    fn example3() {
        assert_eq!(
            most_frequent_prime(vec![vec![9, 7, 8], vec![4, 6, 5], vec![2, 8, 6]]),
            97
        );
    }
}
