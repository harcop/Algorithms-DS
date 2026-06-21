/// LeetCode #2019 - The Score of Students Solving Math Expression
use std::collections::{HashMap, HashSet};

fn score_of_students(s: String, answers: Vec<i32>) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let m = (n + 1) / 2;

    let cal = |s: &[u8]| -> i32 {
        let mut res = 0;
        let mut pre = (s[0] - b'0') as i32;
        let mut i = 1;
        while i < n {
            if s[i] == b'*' {
                pre *= (s[i + 1] - b'0') as i32;
            } else {
                res += pre;
                pre = (s[i + 1] - b'0') as i32;
            }
            i += 2;
        }
        res + pre
    };

    let x = cal(s);
    let mut f = vec![vec![HashSet::new(); m]; m];
    for i in 0..m {
        f[i][i].insert((s[2 * i] - b'0') as i32);
    }

    for i in (0..m).rev() {
        for j in i..m {
            for k in i..j {
                let op = s[2 * k + 1];
                let left = f[i][k].clone();
                let right = f[k + 1][j].clone();
                for &l in &left {
                    for &r in &right {
                        if op == b'+' && l + r <= 1000 {
                            f[i][j].insert(l + r);
                        } else if op == b'*' && l * r <= 1000 {
                            f[i][j].insert(l * r);
                        }
                    }
                }
            }
        }
    }

    let mut cnt = HashMap::new();
    for a in answers {
        *cnt.entry(a).or_insert(0) += 1;
    }

    let mut ans = cnt.get(&x).copied().unwrap_or(0) * 5;
    for (&k, &v) in &cnt {
        if k != x && f[0][m - 1].contains(&k) {
            ans += v * 2;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        score_of_students("7+3*1*2".into(), vec![20, 13, 42])
    );
}

#[cfg(test)]
mod tests {
    use super::score_of_students;

    #[test]
    fn example_one() {
        assert_eq!(score_of_students("7+3*1*2".into(), vec![20, 13, 42]), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            score_of_students("3+5*2".into(), vec![13, 0, 10, 13, 13, 16, 16]),
            19
        );
    }
}
