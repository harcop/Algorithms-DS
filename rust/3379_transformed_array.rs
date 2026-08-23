/// LeetCode #3379 - Transformed Array
fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len() as i32;
    (0..nums.len())
        .map(|i| {
            let idx = (i as i32 + nums[i]).rem_euclid(n) as usize;
            nums[idx]
        })
        .collect()
}

fn main() {
    println!("{:?}", construct_transformed_array(vec![3, -2, 1, 1]));
}

#[cfg(test)]
mod tests {
    use super::construct_transformed_array;

    #[test]
    fn example1() {
        assert_eq!(
            construct_transformed_array(vec![3, -2, 1, 1]),
            vec![1, 1, 1, 3]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            construct_transformed_array(vec![-1, 4, -1]),
            vec![-1, -1, 4]
        );
    }
}
