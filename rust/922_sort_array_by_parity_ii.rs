/// LeetCode #922 - Sort Array By Parity II
fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut j = 1usize;
    for i in (0..n).step_by(2) {
        if nums[i] % 2 != 0 {
            while nums[j] % 2 != 0 {
                j += 2;
            }
            nums.swap(i, j);
        }
    }
    nums
}

fn main() {
    println!("{:?}", sort_array_by_parity_ii(vec![4, 2, 5, 7]));
}

#[cfg(test)]
mod tests {
    use super::sort_array_by_parity_ii;

    #[test]
    fn example_one() {
        let out = sort_array_by_parity_ii(vec![4, 2, 5, 7]);
        for (i, &v) in out.iter().enumerate() {
            assert_eq!(v % 2, i as i32 % 2);
        }
    }

    #[test]
    fn example_two() {
        let out = sort_array_by_parity_ii(vec![2, 3]);
        for (i, &v) in out.iter().enumerate() {
            assert_eq!(v % 2, i as i32 % 2);
        }
    }
}
