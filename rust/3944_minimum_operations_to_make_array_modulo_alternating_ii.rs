/// LeetCode #3944 - Minimum Operations to Make Array Modulo Alternating II
fn add_range(raw: &mut [i64], k: usize, l: usize, r: usize, val: i64) {
    if l > r {
        return;
    }
    raw[l] += val;
    if r + 1 < k {
        raw[r + 1] -= val;
    }
}

fn add_circ(raw: &mut [i64], k: usize, start: usize, end: usize, val: i64) {
    if end < start {
        return;
    }
    let a = start % k;
    let b = end % k;
    if a <= b && end - start == b - a {
        add_range(raw, k, a, b, val);
    } else {
        add_range(raw, k, a, k - 1, val);
        add_range(raw, k, 0, b, val);
    }
}

fn residue_costs(freq: &[i64], k: usize) -> Vec<i64> {
    let mut raw = vec![0i64; k];
    let half = k / 2;
    for r in 0..k {
        let f = freq[r];
        if f == 0 {
            continue;
        }
        add_circ(&mut raw, k, r + 1, r + half, f);
        let start_off = if k % 2 == 0 { half + 1 } else { half + 2 };
        if start_off <= k - 1 {
            add_circ(&mut raw, k, r + start_off, r + (k - 1), -f);
        }
        add_range(&mut raw, k, r, r, -f);
    }
    let mut delta = vec![0i64; k];
    delta[0] = raw[0];
    for i in 1..k {
        delta[i] = delta[i - 1] + raw[i];
    }
    let mut cost = vec![0i64; k];
    cost[0] = (0..k).map(|r| freq[r] * (r.min(k - r) as i64)).sum();
    for i in 1..k {
        cost[i] = cost[i - 1] + delta[i];
    }
    cost
}

fn min_operations(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    let mut even = vec![0i64; k];
    let mut odd = vec![0i64; k];
    for (i, v) in nums.iter().enumerate() {
        let r = (*v as usize) % k;
        if i % 2 == 0 {
            even[r] += 1;
        } else {
            odd[r] += 1;
        }
    }
    let a = residue_costs(&even, k);
    let b = residue_costs(&odd, k);
    let mut a_min = 0usize;
    let mut b_min = 0usize;
    for i in 1..k {
        if a[i] < a[a_min] {
            a_min = i;
        }
        if b[i] < b[b_min] {
            b_min = i;
        }
    }
    if a_min != b_min {
        return a[a_min] + b[b_min];
    }
    let mut a2 = i64::MAX;
    let mut b2 = i64::MAX;
    for i in 0..k {
        if i != a_min {
            a2 = a2.min(a[i]);
        }
        if i != b_min {
            b2 = b2.min(b[i]);
        }
    }
    (a[a_min] + b2).min(a2 + b[b_min])
}

fn main() {
    println!("{}", min_operations(vec![1, 4, 2, 8], 3));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![1, 4, 2, 8], 3), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![1, 1, 1], 3), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(min_operations(vec![6, 7, 8], 2), 0);
    }
}
