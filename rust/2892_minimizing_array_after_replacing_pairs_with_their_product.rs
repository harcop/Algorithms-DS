/// LeetCode #2892 - Minimizing Array After Replacing Pairs With Their Product
fn min_array_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut count = 0;
    let mut prod: i64 = -1;
    let k = k as i64;

    for num in nums {
        if num == 0 {
            return 1;
        }
        if prod != -1 && prod * num as i64 <= k {
            prod *= num as i64;
        } else {
            prod = num as i64;
            count += 1;
        }
    }
    count
}

fn main() {
    println!("{}", min_array_length(vec![2, 3, 3, 7, 3, 5], 20));
}

#[cfg(test)]
mod tests {
    use super::min_array_length;

    #[test]
    fn example_one() {
        assert_eq!(min_array_length(vec![2, 3, 3, 7, 3, 5], 20), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_array_length(vec![3, 3, 3, 3], 6), 4);
    }
}
