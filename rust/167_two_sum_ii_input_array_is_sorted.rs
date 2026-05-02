/// LeetCode #167 - Two Sum II - Input Array Is Sorted
fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut lo = 0usize;
    let mut hi = numbers.len() - 1;
    while lo < hi {
        let s = numbers[lo] + numbers[hi];
        match s.cmp(&target) {
            std::cmp::Ordering::Equal => return vec![lo as i32 + 1, hi as i32 + 1],
            std::cmp::Ordering::Less => lo += 1,
            std::cmp::Ordering::Greater => hi -= 1,
        }
    }
    vec![]
}

fn main() {
    println!("{:?}", two_sum(vec![2, 7, 11, 15], 9));
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn example_one() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }

    #[test]
    fn example_two() {
        assert_eq!(two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    }

    #[test]
    fn example_three() {
        assert_eq!(two_sum(vec![-1, 0], -1), vec![1, 2]);
    }
}
