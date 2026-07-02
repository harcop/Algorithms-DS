/// LeetCode #2208 - Minimum Operations to Halve Array Sum
use std::collections::BinaryHeap;

fn halve_array(nums: Vec<i32>) -> i32 {
    let half_sum: f64 = nums.iter().map(|&x| x as f64).sum::<f64>() / 2.0;
    let mut running = 0.0f64;
    let mut ans = 0i32;
    let mut heap: BinaryHeap<i64> = nums.into_iter().map(|x| x as i64).collect();

    while running < half_sum {
        let max_value = heap.pop().unwrap() as f64 / 2.0;
        running += max_value;
        heap.push(max_value as i64);
        ans += 1;
    }

    ans
}

fn main() {
    println!("{}", halve_array(vec![5, 19, 8, 1]));
}

#[cfg(test)]
mod tests {
    use super::halve_array;

    #[test]
    fn example_one() {
        assert_eq!(halve_array(vec![5, 19, 8, 1]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(halve_array(vec![3, 8, 20]), 3);
    }
}
