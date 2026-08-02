/// LeetCode #2910 - Minimum Number of Groups to Create a Valid Assignment
fn min_groups_for_valid_assignment(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut count = HashMap::new();
    for num in nums {
        *count.entry(num).or_insert(0) += 1;
    }
    let min_freq = *count.values().min().unwrap();

    for group_size in (1..=min_freq).rev() {
        let num_groups = get_num_groups(&count, group_size);
        if num_groups > 0 {
            return num_groups;
        }
    }
    unreachable!()
}

fn get_num_groups(count: &std::collections::HashMap<i32, i32>, group_size: i32) -> i32 {
    let mut num_groups = 0;
    for &freq in count.values() {
        let a = freq / (group_size + 1);
        let b = freq % (group_size + 1);
        if b == 0 {
            num_groups += a;
        } else if group_size - b <= a {
            num_groups += a + 1;
        } else {
            return 0;
        }
    }
    num_groups
}

fn main() {
    println!("{}", min_groups_for_valid_assignment(vec![3, 2, 3, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::min_groups_for_valid_assignment;

    #[test]
    fn example_one() {
        assert_eq!(min_groups_for_valid_assignment(vec![3, 2, 3, 2, 3]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            min_groups_for_valid_assignment(vec![10, 10, 10, 3, 1, 1]),
            4
        );
    }
}
