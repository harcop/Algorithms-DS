/// LeetCode #3431 - Minimum Unlocked Indices to Sort Nums
fn min_unlocked_indices(nums: Vec<i32>, locked: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    let mut first2 = n;
    let mut first3 = n;
    let mut last1 = -1i32;
    let mut last2 = -1i32;
    for (i, &x) in nums.iter().enumerate() {
        let i = i as i32;
        if x == 1 {
            last1 = i;
        } else if x == 2 {
            first2 = first2.min(i);
            last2 = i;
        } else {
            first3 = first3.min(i);
        }
    }
    if first3 < last1 {
        return -1;
    }
    let mut ans = 0;
    for (i, &st) in locked.iter().enumerate() {
        let i = i as i32;
        if st != 0 && ((first2 <= i && i < last1) || (first3 <= i && i < last2)) {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        min_unlocked_indices(vec![1, 2, 1, 2, 3, 2], vec![1, 0, 1, 1, 0, 1])
    );
}

#[cfg(test)]
mod tests {
    use super::min_unlocked_indices;

    #[test]
    fn example1() {
        assert_eq!(
            min_unlocked_indices(vec![1, 2, 1, 2, 3, 2], vec![1, 0, 1, 1, 0, 1]),
            0
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_unlocked_indices(vec![1, 2, 1, 1, 3, 2, 2], vec![1, 0, 1, 1, 0, 1, 0]),
            2
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            min_unlocked_indices(vec![1, 2, 1, 2, 3, 2, 1], vec![0, 0, 0, 0, 0, 0, 0]),
            -1
        );
    }
}
