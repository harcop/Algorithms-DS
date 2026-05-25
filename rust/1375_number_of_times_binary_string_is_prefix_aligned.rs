/// LeetCode #1375 - Number Of Times Binary String Is Prefix Aligned

fn num_times_all_blue(flips: Vec<i32>) -> i32 {
    let mut max_seen = 0i32;
    let mut count = 0i32;
    for (i, &f) in flips.iter().enumerate() {
        max_seen = max_seen.max(f);
        if max_seen == (i + 1) as i32 {
            count += 1;
        }
    }
    count
}

fn main() {
    println!("{}", num_times_all_blue(vec![3, 2, 4, 1, 5]));
}

#[cfg(test)]
mod tests {
    use super::num_times_all_blue;

    #[test]
    fn example_one() {
        assert_eq!(num_times_all_blue(vec![3, 2, 4, 1, 5]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_times_all_blue(vec![4, 1, 2, 3]), 1);
    }
}
