/// LeetCode #3069 - Distribute Elements Into Two Arrays I
fn result_array(nums: Vec<i32>) -> Vec<i32> {
    let mut arr1 = vec![nums[0]];
    let mut arr2 = vec![nums[1]];

    for &x in &nums[2..] {
        if arr1.last() > arr2.last() {
            arr1.push(x);
        } else {
            arr2.push(x);
        }
    }

    arr1.extend(arr2);
    arr1
}

fn main() {
    println!("{:?}", result_array(vec![2, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::result_array;

    #[test]
    fn example1() {
        assert_eq!(result_array(vec![2, 1, 3]), vec![2, 3, 1]);
    }

    #[test]
    fn example2() {
        assert_eq!(result_array(vec![5, 4, 3, 8]), vec![5, 3, 4, 8]);
    }
}
