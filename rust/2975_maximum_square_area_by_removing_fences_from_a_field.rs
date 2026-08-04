/// LeetCode #2975 - Maximum Square Area by Removing Fences From a Field
use std::collections::HashSet;

fn maximize_square_area(m: i32, n: i32, mut h_fences: Vec<i32>, mut v_fences: Vec<i32>) -> i32 {
    fn gaps(nums: &mut Vec<i32>, k: i32) -> HashSet<i32> {
        nums.push(1);
        nums.push(k);
        nums.sort_unstable();
        let mut set = HashSet::new();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                set.insert(nums[j] - nums[i]);
            }
        }
        set
    }
    let hs = gaps(&mut h_fences, m);
    let vs = gaps(&mut v_fences, n);
    let ans = hs.intersection(&vs).copied().max().unwrap_or(0);
    if ans == 0 {
        -1
    } else {
        ((ans as i64) * (ans as i64) % 1_000_000_007) as i32
    }
}

fn main() {
    println!("{}", maximize_square_area(4, 3, vec![2, 3], vec![2]));
}

#[cfg(test)]
mod tests {
    use super::maximize_square_area;

    #[test]
    fn example_one() {
        assert_eq!(maximize_square_area(4, 3, vec![2, 3], vec![2]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximize_square_area(6, 7, vec![2], vec![4]), -1);
    }
}
