/// LeetCode #169 - Majority Element
fn majority_element(nums: Vec<i32>) -> i32 {
    let mut cand = 0i32;
    let mut cnt = 0;
    for x in nums {
        if cnt == 0 {
            cand = x;
            cnt = 1;
        } else if x == cand {
            cnt += 1;
        } else {
            cnt -= 1;
        }
    }
    cand
}

fn main() {
    println!("{}", majority_element(vec![2, 2, 1, 1, 1, 2, 2]));
}

#[cfg(test)]
mod tests {
    use super::majority_element;

    #[test]
    fn example_one() {
        assert_eq!(majority_element(vec![3, 2, 3]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
