/// LeetCode #849 - Maximize Distance to Closest Person
fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
    let n = seats.len();
    let mut ans = 0;
    let mut last = -1;
    for i in 0..n {
        if seats[i] == 1 {
            if last == -1 {
                ans = i;
            } else {
                ans = ans.max((i - last as usize) / 2);
            }
            last = i as i32;
        }
    }
    if last != -1 {
        ans = ans.max(n - 1 - last as usize);
    }
    ans as i32
}

fn main() {
    println!("{}", max_dist_to_closest(vec![1, 0, 0, 0, 1, 0, 1]));
}

#[cfg(test)]
mod tests {
    use super::max_dist_to_closest;

    #[test]
    fn example_one() {
        assert_eq!(max_dist_to_closest(vec![1, 0, 0, 0, 1, 0, 1]), 2);
    }
}
