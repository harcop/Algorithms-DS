/// LeetCode #2613 - Beautiful Pairs
use std::collections::HashMap;

fn beautiful_pair(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let n = nums1.len();
    let mut pl: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
    for i in 0..n {
        pl.entry((nums1[i], nums2[i])).or_default().push(i);
    }
    let mut points = Vec::new();
    for i in 0..n {
        let z = (nums1[i], nums2[i]);
        if pl[&z].len() > 1 {
            return vec![i as i32, pl[&z][1] as i32];
        }
        points.push([nums1[i], nums2[i], i as i32]);
    }
    points.sort_unstable_by_key(|p| p[0]);

    fn dist(a: &[i32; 3], b: &[i32; 3]) -> i32 {
        (a[0] - b[0]).abs() + (a[1] - b[1]).abs()
    }

    fn better(a: [i32; 3], b: [i32; 3]) -> bool {
        a[0] < b[0] || (a[0] == b[0] && (a[1] < b[1] || (a[1] == b[1] && a[2] < b[2])))
    }

    fn dfs(points: &[[i32; 3]], l: usize, r: usize) -> [i32; 3] {
        if l >= r {
            return [1 << 30, -1, -1];
        }
        let m = (l + r) >> 1;
        let x = points[m][0];
        let mut t1 = dfs(points, l, m);
        let t2 = dfs(points, m + 1, r);
        if better(t2, t1) {
            t1 = t2;
        }
        let mut t: Vec<[i32; 3]> = points[l..=r]
            .iter()
            .copied()
            .filter(|p| (p[0] - x).abs() <= t1[0])
            .collect();
        t.sort_unstable_by_key(|p| p[1]);
        for i in 0..t.len() {
            for j in i + 1..t.len() {
                if t[j][1] - t[i][1] > t1[0] {
                    break;
                }
                let pi = t[i][2].min(t[j][2]);
                let pj = t[i][2].max(t[j][2]);
                let d = dist(&t[i], &t[j]);
                let cand = [d, pi, pj];
                if better(cand, t1) {
                    t1 = cand;
                }
            }
        }
        t1
    }

    let ans = dfs(&points, 0, points.len() - 1);
    vec![ans[1], ans[2]]
}

fn main() {
    println!("{:?}", beautiful_pair(vec![1, 2, 3, 2, 4], vec![2, 3, 1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::beautiful_pair;

    #[test]
    fn example_one() {
        assert_eq!(
            beautiful_pair(vec![1, 2, 3, 2, 4], vec![2, 3, 1, 2, 3]),
            vec![0, 3]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            beautiful_pair(vec![1, 2, 4, 3, 2, 5], vec![1, 4, 2, 3, 5, 1]),
            vec![1, 4]
        );
    }
}
