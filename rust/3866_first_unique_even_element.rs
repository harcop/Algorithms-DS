/// LeetCode #3866 - First Unique Even Element
fn first_unique_even(nums: Vec<i32>) -> i32 {
    let mut cnt = [0i32; 101];
    for &x in &nums {
        cnt[x as usize] += 1;
    }
    for x in nums {
        if x % 2 == 0 && cnt[x as usize] == 1 {
            return x;
        }
    }
    -1
}

fn main() {
    println!("{}", first_unique_even(vec![3, 4, 2, 5, 4, 6]));
}

#[cfg(test)]
mod tests {
    use super::first_unique_even;

    #[test]
    fn example1() {
        assert_eq!(first_unique_even(vec![3, 4, 2, 5, 4, 6]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(first_unique_even(vec![4, 4]), -1);
    }
}
