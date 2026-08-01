/// LeetCode #2871 - Split Array Into Maximum Number of Subarrays
fn max_subarrays(nums: Vec<i32>) -> i32 {
    let mut score = -1;
    let mut answer = 1;

    for number in nums {
        score &= number;
        if score == 0 {
            score = -1;
            answer += 1;
        }
    }

    if answer == 1 {
        1
    } else {
        answer - 1
    }
}

fn main() {
    println!("{}", max_subarrays(vec![1, 0, 2, 0, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::max_subarrays;

    #[test]
    fn example_one() {
        assert_eq!(max_subarrays(vec![1, 0, 2, 0, 1, 2]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_subarrays(vec![5, 7, 1, 3]), 1);
    }
}
