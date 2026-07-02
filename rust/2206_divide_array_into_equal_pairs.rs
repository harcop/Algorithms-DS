/// LeetCode #2206 - Divide Array Into Equal Pairs
fn divide_array(nums: Vec<i32>) -> bool {
    let mut count = [0i32; 501];
    for num in nums {
        count[num as usize] += 1;
    }
    count.iter().all(|&c| c % 2 == 0)
}

fn main() {
    println!("{}", divide_array(vec![3, 2, 3, 2, 2, 2]));
}

#[cfg(test)]
mod tests {
    use super::divide_array;

    #[test]
    fn example_one() {
        assert!(divide_array(vec![3, 2, 3, 2, 2, 2]));
    }

    #[test]
    fn example_two() {
        assert!(!divide_array(vec![1, 2, 3, 4]));
    }
}
