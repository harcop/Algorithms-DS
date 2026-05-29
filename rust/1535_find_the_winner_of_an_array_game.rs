/// LeetCode #1535 - Find The Winner Of An Array Game
fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    if k >= arr.len() - 1 {
        return *arr.iter().max().unwrap();
    }
    let mut top = arr[0];
    let mut wins = 0;
    for &x in arr.iter().skip(1) {
        if x > top {
            top = x;
            wins = 1;
        } else {
            wins += 1;
        }
        if wins == k {
            return top;
        }
    }
    top
}

fn main() {
    println!("{}", get_winner(vec![2, 1, 3, 5, 4, 6], 2));
}

#[cfg(test)]
mod tests {
    use super::get_winner;

    #[test]
    fn example_one() {
        assert_eq!(get_winner(vec![2, 1, 3, 5, 4, 6], 2), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(get_winner(vec![2, 11, 10, 1, 3], 10), 11);
    }
}
