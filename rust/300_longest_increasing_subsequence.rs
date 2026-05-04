/// LeetCode #300 - Longest Increasing Subsequence
fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut tails = vec![];
    for x in nums {
        let p = tails.partition_point(|&t| t < x);
        if p == tails.len() {
            tails.push(x);
        } else {
            tails[p] = x;
        }
    }
    tails.len() as i32
}

fn main() {
    println!("{}", length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]));
}

#[cfg(test)]
mod tests {
    use super::length_of_lis;

    #[test]
    fn example_one() {
        assert_eq!(length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    }
}
