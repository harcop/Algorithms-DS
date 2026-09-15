/// LeetCode #3751 - Total Waviness of Numbers in Range I
fn total_waviness(num1: i32, num2: i32) -> i32 {
    (num1..=num2).map(waviness).sum()
}

fn waviness(mut x: i32) -> i32 {
    let mut nums = Vec::new();
    while x > 0 {
        nums.push(x % 10);
        x /= 10;
    }
    let m = nums.len();
    if m < 3 {
        return 0;
    }
    let mut s = 0;
    for i in 1..m - 1 {
        if (nums[i] > nums[i - 1] && nums[i] > nums[i + 1])
            || (nums[i] < nums[i - 1] && nums[i] < nums[i + 1])
        {
            s += 1;
        }
    }
    s
}

fn main() {
    println!("{}", total_waviness(120, 130));
}

#[cfg(test)]
mod tests {
    use super::total_waviness;

    #[test]
    fn example1() {
        assert_eq!(total_waviness(120, 130), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(total_waviness(198, 202), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(total_waviness(4848, 4848), 2);
    }
}
