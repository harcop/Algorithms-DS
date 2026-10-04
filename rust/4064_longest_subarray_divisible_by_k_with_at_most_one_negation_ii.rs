/// LeetCode #4064 - Longest Subarray Divisible by K with At Most One Negation II
fn longest_subarray(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    let norm = |x: i32| -> usize {
        let r = x % k as i32;
        if r < 0 { (r + k as i32) as usize } else { r as usize }
    };

    let mut pref = vec![0usize; n + 1];
    for i in 0..n {
        pref[i + 1] = (pref[i] + norm(nums[i])) % k;
    }

    let mut first = vec![-1i32; k];
    for (i, &p) in pref.iter().enumerate() {
        if first[p] == -1 {
            first[p] = i as i32;
        }
    }

    let mut order: Vec<usize> = (0..k).filter(|&q| first[q] != -1).collect();
    order.sort_by_key(|&q| first[q]);

    let sentinel = (n + 1) as i32;
    let mut pos = vec![0usize; k];
    let mut best = vec![sentinel; k];
    for q in 0..k {
        if first[q] != -1 {
            best[q] = first[q];
        }
    }

    let mut ans = 0i32;
    for i in 0..n {
        let a = norm(nums[i]);
        while pos[a] < order.len() && first[order[pos[a]]] <= i as i32 {
            let q = order[pos[a]];
            pos[a] += 1;
            let t = (q + 2 * a) % k;
            best[t] = best[t].min(first[q]);
        }
        let s = pref[i + 1];
        if best[s] != sentinel {
            ans = ans.max(i as i32 + 1 - best[s]);
        }
    }
    ans
}

fn main() {
    println!("{}", longest_subarray(vec![4, 1, 2], 3));
}

#[cfg(test)]
mod tests {
    use super::longest_subarray;

    #[test]
    fn example1() {
        assert_eq!(longest_subarray(vec![4, 1, 2], 3), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(longest_subarray(vec![5, 3, 4], 7), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(longest_subarray(vec![2, 2, 5], 6), 2);
    }

    #[test]
    fn negative_values() {
        assert_eq!(longest_subarray(vec![-2, 4, 1], 3), 3);
    }
}
