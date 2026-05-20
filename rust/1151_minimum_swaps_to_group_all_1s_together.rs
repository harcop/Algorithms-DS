/// LeetCode #1151 - Minimum Swaps to Group All 1's Together
fn min_swaps(data: Vec<i32>) -> i32 {
    let ones = data.iter().filter(|&&x| x == 1).count();
    if ones == 0 {
        return 0;
    }
    let mut window_ones = 0i32;
    let mut best = i32::MAX;
    let mut left = 0usize;
    for right in 0..data.len() {
        if data[right] == 1 {
            window_ones += 1;
        }
        while right - left + 1 > ones {
            if data[left] == 1 {
                window_ones -= 1;
            }
            left += 1;
        }
        if right - left + 1 == ones {
            best = best.min((ones as i32) - window_ones);
        }
    }
    best
}

fn main() {
    println!("{}", min_swaps(vec![1, 0, 1, 0, 1]));
}

#[cfg(test)]
mod tests {
    use super::min_swaps;

    #[test]
    fn example_one() {
        assert_eq!(min_swaps(vec![1, 0, 1, 0, 1]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_swaps(vec![0, 0, 0, 1, 0]), 0);
    }
}
