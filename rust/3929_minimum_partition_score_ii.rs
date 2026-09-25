/// LeetCode #3929 - Minimum Partition Score II
use std::collections::VecDeque;

#[derive(Clone, Copy)]
struct Line {
    m: i128,
    b: i128,
    seg: i32,
}

fn eval(line: Line, x: i128) -> i128 {
    line.b + line.m * x
}

/// `l2` sits strictly above the chord from `l1` to `l3`.
fn redundant(l1: Line, l2: Line, l3: Line) -> bool {
    (l2.b - l1.b) * (l2.m - l3.m) > (l3.b - l2.b) * (l1.m - l2.m)
}

/// Minimum penalized cost, preferring more segments on ties.
fn check(pref: &[i128], lambda: i128) -> (i128, i32) {
    let n = pref.len() - 1;
    let mut hull = VecDeque::new();
    hull.push_back(Line { m: 0, b: 0, seg: 0 });
    let mut cost = 0i128;
    let mut seg = 0i32;
    for i in 1..=n {
        let x = pref[i];
        while hull.len() >= 2 {
            let e0 = eval(hull[0], x);
            let e1 = eval(hull[1], x);
            if e1 < e0 || (e1 == e0 && hull[1].seg >= hull[0].seg) {
                hull.pop_front();
            } else {
                break;
            }
        }
        let best = hull[0];
        cost = eval(best, x) + x * (x + 1) / 2 + lambda;
        seg = best.seg + 1;
        let line = Line {
            m: -x,
            b: cost + x * (x - 1) / 2,
            seg,
        };
        while hull.len() >= 2 {
            let nline = hull.len();
            if redundant(hull[nline - 2], hull[nline - 1], line) {
                hull.pop_back();
            } else {
                break;
            }
        }
        hull.push_back(line);
    }
    (cost, seg)
}

fn minimum_partition_score(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let mut pref = vec![0i128; n + 1];
    for i in 0..n {
        pref[i + 1] = pref[i] + nums[i] as i128;
    }
    let total = pref[n];
    let mut lo = 0i128;
    let mut hi = total * total;
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        let (_, seg) = check(&pref, mid);
        if seg >= k {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    let (cost, _) = check(&pref, lo);
    (cost - lo * k as i128) as i64
}

fn main() {
    println!("{}", minimum_partition_score(vec![5, 1, 2, 1], 2));
}

#[cfg(test)]
mod tests {
    use super::minimum_partition_score;

    #[test]
    fn example1() {
        assert_eq!(minimum_partition_score(vec![5, 1, 2, 1], 2), 25);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_partition_score(vec![1, 2, 3, 4], 1), 55);
    }

    #[test]
    fn example3() {
        assert_eq!(minimum_partition_score(vec![1, 1, 1], 3), 3);
    }
}
