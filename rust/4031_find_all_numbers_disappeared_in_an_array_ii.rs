/// LeetCode #4031 - Find All Numbers Disappeared in an Array II
fn find_disappeared_numbers(mut nums: Vec<i32>, lower: i32, upper: i32) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    nums.dedup();
    let mut ans = Vec::new();
    let mut prev = lower - 1;
    for x in nums {
        if x < lower {
            continue;
        }
        if x > upper {
            break;
        }
        if x - prev > 1 {
            ans.push(vec![prev + 1, x - 1]);
        }
        prev = x;
    }
    if prev < upper {
        ans.push(vec![prev + 1, upper]);
    }
    ans
}

fn main() {
    println!("{:?}", find_disappeared_numbers(vec![3, 9, 7], 1, 12));
}

#[cfg(test)]
mod tests {
    use super::find_disappeared_numbers;

    #[test]
    fn example1() {
        assert_eq!(
            find_disappeared_numbers(vec![3, 9, 7], 1, 12),
            vec![vec![1, 2], vec![4, 6], vec![8, 8], vec![10, 12]]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            find_disappeared_numbers(vec![1, 1], 5, 7),
            vec![vec![5, 7]]
        );
    }

    #[test]
    fn example3() {
        assert!(find_disappeared_numbers(vec![2, 3, 5], 2, 3).is_empty());
    }
}
