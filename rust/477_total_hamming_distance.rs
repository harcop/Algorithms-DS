/// LeetCode #477 - Total Hamming Distance
fn total_hamming_distance(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    let mut ans = 0i32;
    for b in 0..31 {
        let mut ones = 0i32;
        for &x in &nums {
            ones += (x >> b) & 1;
        }
        ans += ones * (n - ones);
    }
    ans
}

fn main() {
    println!("{}", total_hamming_distance(vec![4, 14, 2]));
}

#[cfg(test)]
mod tests {
    use super::total_hamming_distance;

    #[test]
    fn example_one() {
        assert_eq!(total_hamming_distance(vec![4, 14, 2]), 6);
    }
}
