/// LeetCode #3321 - Find X-Sum of All K-Long Subarrays II
use std::collections::{BTreeSet, HashMap};

fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
    let k = k as usize;
    let x = x as usize;
    let n = nums.len();
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut l: BTreeSet<(i32, i32)> = BTreeSet::new();
    let mut r: BTreeSet<(i32, i32)> = BTreeSet::new();
    let mut s: i64 = 0;

    let add = |v: i32,
               cnt: &HashMap<i32, i32>,
               l: &mut BTreeSet<(i32, i32)>,
               r: &mut BTreeSet<(i32, i32)>,
               s: &mut i64| {
        let c = *cnt.get(&v).unwrap_or(&0);
        if c == 0 {
            return;
        }
        let p = (c, v);
        if let Some(&lo) = l.iter().next() {
            if p > lo {
                *s += p.0 as i64 * p.1 as i64;
                l.insert(p);
                return;
            }
        }
        r.insert(p);
    };

    let remove = |v: i32,
                  cnt: &HashMap<i32, i32>,
                  l: &mut BTreeSet<(i32, i32)>,
                  r: &mut BTreeSet<(i32, i32)>,
                  s: &mut i64| {
        let c = *cnt.get(&v).unwrap_or(&0);
        if c == 0 {
            return;
        }
        let p = (c, v);
        if l.remove(&p) {
            *s -= p.0 as i64 * p.1 as i64;
        } else {
            r.remove(&p);
        }
    };

    let mut ans = vec![0i64; n - k + 1];
    for i in 0..n {
        let v = nums[i];
        remove(v, &cnt, &mut l, &mut r, &mut s);
        *cnt.entry(v).or_insert(0) += 1;
        add(v, &cnt, &mut l, &mut r, &mut s);
        if i + 1 < k {
            continue;
        }
        let j = i + 1 - k;
        while !r.is_empty() && l.len() < x {
            let p = r.pop_last().unwrap();
            l.insert(p);
            s += p.0 as i64 * p.1 as i64;
        }
        while l.len() > x {
            let p = l.pop_first().unwrap();
            s -= p.0 as i64 * p.1 as i64;
            r.insert(p);
        }
        ans[j] = s;

        let left = nums[j];
        remove(left, &cnt, &mut l, &mut r, &mut s);
        *cnt.get_mut(&left).unwrap() -= 1;
        add(left, &cnt, &mut l, &mut r, &mut s);
    }
    ans
}

fn main() {
    println!("{:?}", find_x_sum(vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2));
}

#[cfg(test)]
mod tests {
    use super::find_x_sum;

    #[test]
    fn example1() {
        assert_eq!(
            find_x_sum(vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2),
            vec![6, 10, 12]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            find_x_sum(vec![3, 8, 7, 8, 7, 5], 2, 2),
            vec![11, 15, 15, 15, 12]
        );
    }
}
