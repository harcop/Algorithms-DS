/// LeetCode #1982 - Find Array Given Subset Sums
use std::collections::BTreeMap;

fn remove_one(cnt: &mut BTreeMap<i32, i32>, key: i32) {
    if let Some(v) = cnt.get_mut(&key) {
        *v -= 1;
        if *v == 0 {
            cnt.remove(&key);
        }
    }
}

fn recover_array(n: i32, sums: Vec<i32>) -> Vec<i32> {
    let n = n as usize;
    let m = -sums.iter().copied().min().unwrap();
    let mut cnt: BTreeMap<i32, i32> = BTreeMap::new();
    for x in sums {
        *cnt.entry(x + m).or_insert(0) += 1;
    }
    remove_one(&mut cnt, 0);
    let mut ans = vec![*cnt.keys().next().unwrap()];

    for i in 1..n {
        for j in 0..(1usize << i) {
            if (j >> (i - 1)) & 1 == 1 {
                let s: i32 = (0..i)
                    .filter(|&k| (j >> k) & 1 == 1)
                    .map(|k| ans[k])
                    .sum();
                remove_one(&mut cnt, s);
            }
        }
        ans.push(*cnt.keys().next().unwrap());
    }

    for i in 0..(1usize << n) {
        let s: i32 = (0..n)
            .filter(|&j| (i >> j) & 1 == 1)
            .map(|j| ans[j])
            .sum();
        if s == m {
            for j in 0..n {
                if (i >> j) & 1 == 1 {
                    ans[j] *= -1;
                }
            }
            break;
        }
    }
    ans
}

fn main() {
    println!("{:?}", recover_array(3, vec![-3, -2, -1, 0, 0, 1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::recover_array;

    fn is_valid(n: i32, sums: &[i32], ans: &[i32]) {
        let n = n as usize;
        let mut got: Vec<i32> = (0..(1usize << n))
            .map(|i| (0..n).filter(|&j| (i >> j) & 1 == 1).map(|j| ans[j]).sum())
            .collect();
        got.sort_unstable();
        let mut exp = sums.to_vec();
        exp.sort_unstable();
        assert_eq!(got, exp);
    }

    #[test]
    fn example_one() {
        let ans = recover_array(3, vec![-3, -2, -1, 0, 0, 1, 2, 3]);
        is_valid(3, &[-3, -2, -1, 0, 0, 1, 2, 3], &ans);
    }

    #[test]
    fn example_two() {
        assert_eq!(recover_array(2, vec![0, 0, 0, 0]), vec![0, 0]);
    }
}
