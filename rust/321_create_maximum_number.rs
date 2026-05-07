/// LeetCode #321 - Create Maximum Number
fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n1 = nums1.len();
    let n2 = nums2.len();
    let mut best: Vec<i32> = vec![0; k];
    let lo = k.saturating_sub(n2);
    let hi = n1.min(k);
    for i in lo..=hi {
        let j = k - i;
        if j > n2 {
            continue;
        }
        let a = pick_max_subsequence(&nums1, i);
        let b = pick_max_subsequence(&nums2, j);
        let merged = merge_max(&a, &b);
        if merged > best {
            best = merged;
        }
    }
    best
}

fn pick_max_subsequence(nums: &[i32], k: usize) -> Vec<i32> {
    if k == 0 {
        return vec![];
    }
    let mut drop = nums.len() - k;
    let mut stack: Vec<i32> = Vec::new();
    for &x in nums {
        while drop > 0 && !stack.is_empty() && *stack.last().unwrap() < x {
            stack.pop();
            drop -= 1;
        }
        stack.push(x);
    }
    stack.truncate(k);
    stack
}

fn merge_max(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut i = 0usize;
    let mut j = 0usize;
    let mut out = Vec::with_capacity(a.len() + b.len());
    while i < a.len() || j < b.len() {
        if greater_slice(a, i, b, j) {
            out.push(a[i]);
            i += 1;
        } else {
            out.push(b[j]);
            j += 1;
        }
    }
    out
}

fn greater_slice(a: &[i32], i: usize, b: &[i32], j: usize) -> bool {
    let mut ii = i;
    let mut jj = j;
    while ii < a.len() && jj < b.len() {
        if a[ii] != b[jj] {
            return a[ii] > b[jj];
        }
        ii += 1;
        jj += 1;
    }
    ii < a.len()
}

fn main() {
    println!(
        "{:?}",
        max_number(vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5)
    );
}

#[cfg(test)]
mod tests {
    use super::max_number;

    #[test]
    fn example_one() {
        assert_eq!(
            max_number(vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5),
            vec![9, 8, 6, 5, 3]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            max_number(vec![6, 7], vec![6, 0, 4], 5),
            vec![6, 7, 6, 0, 4]
        );
    }
}
