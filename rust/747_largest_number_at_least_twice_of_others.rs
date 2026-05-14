/// LeetCode #747 - Largest Number At Least Twice of Others
fn dominant_index(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return 0;
    }
    let mut mx = -1i32;
    let mut sm = -1i32;
    let mut idx = 0usize;
    for (i, &x) in nums.iter().enumerate() {
        if x > mx {
            sm = mx;
            mx = x;
            idx = i;
        } else if x > sm {
            sm = x;
        }
    }
    if mx >= 2 * sm {
        idx as i32
    } else {
        -1
    }
}

fn main() {
    println!("{}", dominant_index(vec![3, 6, 1, 0]));
}

#[cfg(test)]
mod tests {
    use super::dominant_index;

    #[test]
    fn example_one() {
        assert_eq!(dominant_index(vec![3, 6, 1, 0]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(dominant_index(vec![1, 2, 3, 4]), -1);
    }
}
