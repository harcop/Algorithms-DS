/// LeetCode #2598 - Smallest Missing Non-negative Integer After Operations
fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
    let value = value as usize;
    let mut cnt = vec![0i32; value];
    for x in nums {
        let idx = ((x % value as i32 + value as i32) % value as i32) as usize;
        cnt[idx] += 1;
    }
    let mut i = 0i32;
    loop {
        let idx = (i as usize) % value;
        if cnt[idx] == 0 {
            return i;
        }
        cnt[idx] -= 1;
        i += 1;
    }
}

fn main() {
    println!(
        "{}",
        find_smallest_integer(vec![1, -10, 7, 13, 6, 8], 5)
    );
}

#[cfg(test)]
mod tests {
    use super::find_smallest_integer;

    #[test]
    fn example_one() {
        assert_eq!(
            find_smallest_integer(vec![1, -10, 7, 13, 6, 8], 5),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_smallest_integer(vec![1, -10, 7, 13, 6, 8], 7),
            2
        );
    }
}
