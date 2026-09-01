/// LeetCode #3523 - Make Array Non-decreasing
fn maximum_possible_size(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut mx = 0;
    for x in nums {
        if mx <= x {
            ans += 1;
            mx = x;
        }
    }
    ans
}

fn main() {
    println!("{}", maximum_possible_size(vec![4, 2, 5, 3, 5]));
}

#[cfg(test)]
mod tests {
    use super::maximum_possible_size;

    #[test]
    fn example1() {
        assert_eq!(maximum_possible_size(vec![4, 2, 5, 3, 5]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_possible_size(vec![1, 2, 3]), 3);
    }
}
