/// LeetCode #1798 - Maximum Number of Consecutive Values You Can Make
fn get_maximum_consecutive(coins: Vec<i32>) -> i32 {
    let mut coins = coins;
    coins.sort_unstable();
    let mut ans = 1i64;
    for v in coins {
        if v as i64 > ans {
            break;
        }
        ans += v as i64;
    }
    ans as i32
}

fn main() {
    println!("{}", get_maximum_consecutive(vec![1, 3]));
}

#[cfg(test)]
mod tests {
    use super::get_maximum_consecutive;

    #[test]
    fn example_one() {
        assert_eq!(get_maximum_consecutive(vec![1, 3]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(get_maximum_consecutive(vec![1, 1, 1, 4]), 8);
    }

    #[test]
    fn example_three() {
        assert_eq!(get_maximum_consecutive(vec![1, 4, 10, 3, 1]), 20);
    }
}
