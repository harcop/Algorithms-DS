/// LeetCode #2832 - Maximal Range That Each Element Is Maximum in It
fn maximum_length_of_ranges(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut left = vec![-1i32; n];
    let mut right = vec![n as i32; n];
    let mut stk: Vec<usize> = vec![];

    for i in 0..n {
        while let Some(&top) = stk.last() {
            if nums[top] <= nums[i] {
                stk.pop();
            } else {
                break;
            }
        }
        if let Some(&top) = stk.last() {
            left[i] = top as i32;
        }
        stk.push(i);
    }

    stk.clear();
    for i in (0..n).rev() {
        while let Some(&top) = stk.last() {
            if nums[top] <= nums[i] {
                stk.pop();
            } else {
                break;
            }
        }
        if let Some(&top) = stk.last() {
            right[i] = top as i32;
        }
        stk.push(i);
    }

    (0..n)
        .map(|i| right[i] - left[i] - 1)
        .collect()
}

fn main() {
    println!("{:?}", maximum_length_of_ranges(vec![1, 5, 4, 3, 6]));
}

#[cfg(test)]
mod tests {
    use super::maximum_length_of_ranges;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_length_of_ranges(vec![1, 5, 4, 3, 6]),
            vec![1, 4, 2, 1, 5]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            maximum_length_of_ranges(vec![1, 2, 3, 4, 5]),
            vec![1, 2, 3, 4, 5]
        );
    }
}
