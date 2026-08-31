/// LeetCode #3510 - Minimum Pair Removal to Sort Array II
use std::collections::BTreeSet;

fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n <= 1 {
        return 0;
    }
    let mut a: Vec<i64> = nums.iter().map(|&x| x as i64).collect();
    let mut prev: Vec<Option<usize>> = (0..n).map(|i| i.checked_sub(1)).collect();
    let mut next: Vec<Option<usize>> = (0..n)
        .map(|i| if i + 1 < n { Some(i + 1) } else { None })
        .collect();
    let mut pairs: BTreeSet<(i64, usize)> = BTreeSet::new();
    let mut inv = 0i32;
    for i in 0..n - 1 {
        pairs.insert((a[i] + a[i + 1], i));
        if a[i] > a[i + 1] {
            inv += 1;
        }
    }
    let mut ans = 0;
    while inv > 0 {
        ans += 1;
        let (s, i) = *pairs.iter().next().unwrap();
        pairs.remove(&(s, i));
        let j = next[i].unwrap();
        if a[i] > a[j] {
            inv -= 1;
        }
        if let Some(h) = prev[i] {
            if a[h] > a[i] {
                inv -= 1;
            }
            pairs.remove(&(a[h] + a[i], h));
        }
        if let Some(k) = next[j] {
            if a[j] > a[k] {
                inv -= 1;
            }
            pairs.remove(&(a[j] + a[k], j));
        }
        a[i] = s;
        next[i] = next[j];
        if let Some(k) = next[j] {
            prev[k] = Some(i);
        }
        next[j] = None;
        prev[j] = None;
        if let Some(h) = prev[i] {
            if a[h] > a[i] {
                inv += 1;
            }
            pairs.insert((a[h] + a[i], h));
        }
        if let Some(k) = next[i] {
            if a[i] > a[k] {
                inv += 1;
            }
            pairs.insert((a[i] + a[k], i));
        }
    }
    ans
}

fn main() {
    println!("{}", minimum_pair_removal(vec![5, 2, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::minimum_pair_removal;

    #[test]
    fn example1() {
        assert_eq!(minimum_pair_removal(vec![5, 2, 3, 1]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_pair_removal(vec![1, 2, 2]), 0);
    }
}
