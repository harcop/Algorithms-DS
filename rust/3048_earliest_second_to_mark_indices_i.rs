/// LeetCode #3048 - Earliest Second to Mark Indices I
use std::collections::HashMap;

fn can_mark(nums: &[i32], change_indices: &[i32], t: usize) -> bool {
    let n = nums.len();
    let slice = &change_indices[..t];
    let mut last = HashMap::new();
    for (s, &idx) in slice.iter().enumerate() {
        last.insert(idx, s);
    }

    let mut decrement = 0i32;
    let mut marked = 0;
    for (s, &idx) in slice.iter().enumerate() {
        if last[&idx] == s {
            let need = nums[(idx - 1) as usize];
            if decrement < need {
                return false;
            }
            decrement -= need;
            marked += 1;
        } else {
            decrement += 1;
        }
    }
    marked == n
}

fn earliest_second_to_mark_indices_i(nums: Vec<i32>, change_indices: Vec<i32>) -> i32 {
    let m = change_indices.len();
    let mut lo = 1i32;
    let mut hi = m as i32;
    let mut ans = -1;

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if can_mark(&nums, &change_indices, mid as usize) {
            ans = mid;
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }

    ans
}

fn main() {
    println!(
        "{}",
        earliest_second_to_mark_indices_i(vec![2, 2, 0], vec![2, 2, 2, 2, 3, 2, 2, 1])
    );
}

#[cfg(test)]
mod tests {
    use super::earliest_second_to_mark_indices_i;

    #[test]
    fn example1() {
        assert_eq!(
            earliest_second_to_mark_indices_i(vec![2, 2, 0], vec![2, 2, 2, 2, 3, 2, 2, 1]),
            8
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            earliest_second_to_mark_indices_i(vec![1, 3], vec![1, 1, 1, 2, 1, 1, 1]),
            6
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            earliest_second_to_mark_indices_i(vec![0, 1], vec![2, 2, 2]),
            -1
        );
    }
}
