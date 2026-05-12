/// LeetCode #704 - Binary Search
fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut lo = 0i32;
    let mut hi = nums.len() as i32 - 1;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if nums[mid as usize] == target { return mid; }
        if nums[mid as usize] < target { lo = mid + 1; } else { hi = mid - 1; }
    }
    -1
}

fn main() {
    println!("{}", search(vec![-1,0,3,5,9,12], 9));
}

#[cfg(test)]
mod tests {
    use super::search;

    #[test]
    fn example_one() {
        assert_eq!(search(vec![-1,0,3,5,9,12], 9), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(search(vec![-1,0,3,5,9,12], 2), -1);
    }
}
