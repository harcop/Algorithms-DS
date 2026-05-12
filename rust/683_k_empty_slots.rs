/// LeetCode #683 - K Empty Slots
fn k_empty_slots(bulbs: Vec<i32>, k: i32) -> i32 {
    let n = bulbs.len();
    let mut days = vec![0i32; n];
    for i in 0..n {
        days[(bulbs[i] - 1) as usize] = (i + 1) as i32;
    }
    let span = (k + 1) as usize;
    if span >= n {
        return -1;
    }
    let mut ans = i32::MAX;
    let mut left = 0usize;
    let mut right = span;
    while right < n {
        let mut valid = true;
        let mut new_left = right;
        for i in left + 1..right {
            if days[i] < days[left] || days[i] < days[right] {
                new_left = i;
                valid = false;
                break;
            }
        }
        if valid {
            ans = ans.min(days[left].max(days[right]));
            left = right;
            right += span;
        } else {
            left = new_left;
            right = new_left + span;
        }
    }
    if ans == i32::MAX {
        -1
    } else {
        ans
    }
}

fn main() {
    println!("{}", k_empty_slots(vec![1, 3, 2], 1));
}

#[cfg(test)]
mod tests {
    use super::k_empty_slots;

    #[test]
    fn example_one() {
        assert_eq!(k_empty_slots(vec![1, 3, 2], 1), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(k_empty_slots(vec![1, 2, 3], 1), -1);
    }
}
