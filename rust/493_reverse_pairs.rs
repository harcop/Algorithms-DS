/// LeetCode #493 - Reverse Pairs
fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut tmp = vec![0; n];
    merge_count(&mut nums, &mut tmp, 0, n) as i32
}

fn merge_count(a: &mut [i32], tmp: &mut [i32], l: usize, r: usize) -> i64 {
    if r - l <= 1 {
        return 0;
    }
    let m = (l + r) / 2;
    let mut cnt = merge_count(a, tmp, l, m) + merge_count(a, tmp, m, r);
    let mut j = m;
    for i in l..m {
        while j < r && (a[i] as i64) > 2 * (a[j] as i64) {
            j += 1;
        }
        cnt += (j - m) as i64;
    }
    let mut i = l;
    let mut j2 = m;
    let mut k = l;
    while i < m && j2 < r {
        if a[i] <= a[j2] {
            tmp[k] = a[i];
            i += 1;
        } else {
            tmp[k] = a[j2];
            j2 += 1;
        }
        k += 1;
    }
    while i < m {
        tmp[k] = a[i];
        i += 1;
        k += 1;
    }
    while j2 < r {
        tmp[k] = a[j2];
        j2 += 1;
        k += 1;
    }
    a[l..r].copy_from_slice(&tmp[l..r]);
    cnt
}

fn main() {
    println!("{}", reverse_pairs(vec![1, 3, 2, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::reverse_pairs;

    #[test]
    fn example_one() {
        assert_eq!(reverse_pairs(vec![1, 3, 2, 3, 1]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(reverse_pairs(vec![2, 4, 3, 5, 1]), 3);
    }
}
