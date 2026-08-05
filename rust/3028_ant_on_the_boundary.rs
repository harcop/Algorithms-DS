/// LeetCode #3028 - Ant on the Boundary
fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut count = 0;
    for &x in &nums {
        sum += x;
        if sum == 0 {
            count += 1;
        }
    }
    count
}

fn main() {
    println!("{}", return_to_boundary_count(vec![2, 3, -5]));
}

#[cfg(test)]
mod tests {
    use super::return_to_boundary_count;

    #[test]
    fn example1() {
        assert_eq!(return_to_boundary_count(vec![2, 3, -5]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(return_to_boundary_count(vec![3, 2, -3, -4]), 0);
    }
}
