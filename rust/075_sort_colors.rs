/// LeetCode #75 - Sort Colors (Dutch flag)
fn sort_colors(nums: &mut Vec<i32>) {
    let mut low = 0usize;
    let mut mid = 0usize;
    let mut high = nums.len();

    while mid < high {
        match nums[mid] {
            0 => {
                nums.swap(low, mid);
                low += 1;
                mid += 1;
            }
            1 => mid += 1,
            _ => {
                high -= 1;
                nums.swap(mid, high);
            }
        }
    }
}

fn main() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    sort_colors(&mut nums);
    println!("{nums:?}");
}

#[cfg(test)]
mod tests {
    use super::sort_colors;

    #[test]
    fn example_one() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn example_two() {
        let mut nums = vec![2, 0, 1];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
    }
}
