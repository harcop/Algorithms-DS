/// LeetCode #2562 - Find the Array Concatenation Value
fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
    let mut ans = 0i64;
    let mut i = 0usize;
    let mut j = nums.len() - 1;
    while i <= j {
        if i == j {
            ans += nums[i] as i64;
            break;
        }
        let mut mul = 1i64;
        let mut t = nums[j];
        while t > 0 {
            mul *= 10;
            t /= 10;
        }
        ans += nums[i] as i64 * mul + nums[j] as i64;
        i += 1;
        if j == 0 {
            break;
        }
        j -= 1;
    }
    ans
}

fn main() {
    println!("{}", find_the_array_conc_val(vec![7, 52, 2, 4]));
}

#[cfg(test)]
mod tests {
    use super::find_the_array_conc_val;

    #[test]
    fn example_one() {
        assert_eq!(find_the_array_conc_val(vec![7, 52, 2, 4]), 596);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_the_array_conc_val(vec![5, 14, 13, 8, 12]), 673);
    }
}
