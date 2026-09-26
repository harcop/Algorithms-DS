/// LeetCode #3943 - Number of Pairs After Increment
use std::collections::HashMap;

fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
    let n = nums2.len();
    let b = (n as f64).sqrt().ceil().max(1.0) as usize;
    let m = (n + b - 1) / b;
    let mut a: Vec<i64> = nums2.iter().map(|&x| x as i64).collect();
    let mut lazy = vec![0i64; m];
    let mut freq: Vec<HashMap<i64, i32>> = (0..m)
        .map(|block| {
            let mut c = HashMap::new();
            for i in block * b..((block + 1) * b).min(n) {
                *c.entry(a[i]).or_insert(0) += 1;
            }
            c
        })
        .collect();
    let mut cnt1: HashMap<i64, i64> = HashMap::new();
    for x in nums1 {
        *cnt1.entry(x as i64).or_insert(0) += 1;
    }
    let items1: Vec<(i64, i64)> = cnt1.into_iter().collect();

    let push = |block: usize, a: &mut [i64], lazy: &mut [i64], freq: &mut [HashMap<i64, i32>]| {
        if lazy[block] == 0 {
            return;
        }
        let d = lazy[block];
        let l = block * b;
        let r = ((block + 1) * b).min(n);
        let mut nf = HashMap::new();
        for i in l..r {
            a[i] += d;
            *nf.entry(a[i]).or_insert(0) += 1;
        }
        freq[block] = nf;
        lazy[block] = 0;
    };

    let mut ans = Vec::new();
    for q in queries {
        if q[0] == 1 {
            let x = q[1] as usize;
            let y = q[2] as usize;
            let val = q[3] as i64;
            for block in 0..m {
                let l = block * b;
                let r = ((block + 1) * b).min(n);
                if y < l || r - 1 < x {
                    continue;
                }
                if x <= l && r - 1 <= y {
                    lazy[block] += val;
                } else {
                    push(block, &mut a, &mut lazy, &mut freq);
                    let start = x.max(l);
                    let end = y.min(r - 1);
                    for i in start..=end {
                        let old = a[i];
                        let e = freq[block].get_mut(&old).unwrap();
                        *e -= 1;
                        if *e == 0 {
                            freq[block].remove(&old);
                        }
                        a[i] += val;
                        *freq[block].entry(a[i]).or_insert(0) += 1;
                    }
                }
            }
        } else {
            let tot = q[1] as i64;
            let mut res = 0i64;
            for block in 0..m {
                for &(x, c) in &items1 {
                    let need = tot - x - lazy[block];
                    if let Some(&cnt) = freq[block].get(&need) {
                        res += cnt as i64 * c;
                    }
                }
            }
            ans.push(res);
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        number_of_pairs(
            vec![1, 2],
            vec![3, 4],
            vec![vec![2, 5], vec![1, 0, 0, 2], vec![2, 5]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::number_of_pairs;

    #[test]
    fn example1() {
        assert_eq!(
            number_of_pairs(
                vec![1, 2],
                vec![3, 4],
                vec![vec![2, 5], vec![1, 0, 0, 2], vec![2, 5]]
            ),
            vec![2, 1]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            number_of_pairs(
                vec![1, 1],
                vec![2, 2, 3],
                vec![vec![2, 4], vec![1, 0, 1, 1], vec![2, 4]]
            ),
            vec![2, 6]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            number_of_pairs(
                vec![2, 5, 8, 4],
                vec![1, 3, 8],
                vec![vec![2, 9], vec![1, 1, 2, 1], vec![2, 10]]
            ),
            vec![1, 0]
        );
    }
}
