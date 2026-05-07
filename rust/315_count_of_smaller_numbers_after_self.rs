/// LeetCode #315 - Count of Smaller Numbers After Self (BIT / Fenwick tree)
fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {
        return vec![];
    }
    let mut sorted: Vec<i32> = nums.iter().copied().collect();
    sorted.sort_unstable();
    sorted.dedup();
    let rank = |x: i32| sorted.binary_search(&x).unwrap() as usize + 1;

    let m = sorted.len();
    let mut bit = vec![0i32; m + 2];

    fn bit_add(bit: &mut [i32], mut i: usize, v: i32) {
        while i < bit.len() {
            bit[i] += v;
            i += i & (!i + 1);
        }
    }
    fn bit_sum(bit: &[i32], mut i: usize) -> i32 {
        let mut s = 0i32;
        while i > 0 {
            s += bit[i];
            i -= i & (!i + 1);
        }
        s
    }

    let n = nums.len();
    let mut out = vec![0; n];
    for i in (0..n).rev() {
        let r = rank(nums[i]);
        out[i] = bit_sum(&bit, r - 1);
        bit_add(&mut bit, r, 1);
    }
    out
}

fn main() {
    println!("{:?}", count_smaller(vec![5, 2, 6, 1]));
}

#[cfg(test)]
mod tests {
    use super::count_smaller;

    #[test]
    fn example() {
        assert_eq!(count_smaller(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);
    }

    #[test]
    fn with_dupes() {
        assert_eq!(count_smaller(vec![-1, -1]), vec![0, 0]);
    }
}
