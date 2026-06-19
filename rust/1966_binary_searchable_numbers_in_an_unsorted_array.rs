/// LeetCode #1966 - Binary Searchable Numbers in an Unsorted Array
fn binary_searchable_numbers(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ok = vec![true; n];
    let mut mx = i32::MIN;
    for i in 0..n {
        if nums[i] < mx {
            ok[i] = false;
        } else {
            mx = nums[i];
        }
    }
    let mut mi = i32::MAX;
    for i in (0..n).rev() {
        if nums[i] > mi {
            ok[i] = false;
        } else {
            mi = nums[i];
        }
    }
    ok.iter().filter(|&&v| v).count() as i32
}

fn main() {
    println!("{}", binary_searchable_numbers(vec![3, 1, 5, 4, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::binary_searchable_numbers;

    #[test]
    fn example_one() {
        assert_eq!(binary_searchable_numbers(vec![7]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(binary_searchable_numbers(vec![-1, 5, 2]), 1);
    }
}
