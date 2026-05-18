/// LeetCode #1013 - Partition Array Into Three Parts With Equal Sum
fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
    let sum: i64 = arr.iter().map(|&x| x as i64).sum();
    if sum % 3 != 0 {
        return false;
    }
    let target = sum / 3;
    let mut parts = 0i32;
    let mut cur = 0i64;
    for x in arr {
        cur += x as i64;
        if cur == target {
            parts += 1;
            cur = 0;
        }
    }
    parts >= 3
}

fn main() {
    println!("{}", can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1]));
}

#[cfg(test)]
mod tests {
    use super::can_three_parts_equal_sum;

    #[test]
    fn example_one() {
        assert!(can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1]));
    }

    #[test]
    fn example_two() {
        assert!(!can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1]));
    }
}
