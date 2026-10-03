/// LeetCode #4059 - Lexicographically Largest Power Array
fn largest_power(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable_by(|a, b| b.cmp(a));
    let n = nums.len();
    let mut power = vec![0i32; 15];
    for i in 0..15 {
        let bit = 14 - i;
        let mut j = 0;
        while j < n && (nums[j] >> bit) & 1 == 1 {
            j += 1;
        }
        power[i] = j as i32;
    }
    power
}

fn main() {
    println!("{:?}", largest_power(vec![7, 5]));
}

#[cfg(test)]
mod tests {
    use super::largest_power;

    #[test]
    fn example1() {
        assert_eq!(
            largest_power(vec![7, 5]),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 1, 2]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            largest_power(vec![3, 1, 7]),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3]
        );
    }
}
