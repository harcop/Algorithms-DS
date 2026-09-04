/// LeetCode #517 - Super Washing Machines
fn find_min_moves(machines: Vec<i32>) -> i32 {
    let n = machines.len() as i32;
    let sum: i32 = machines.iter().sum();
    if sum % n != 0 {
        return -1;
    }
    let avg = sum / n;
    let mut ans = 0;
    let mut acc = 0;
    for m in machines {
        acc += m - avg;
        ans = ans.max(acc.abs()).max(m - avg);
    }
    ans
}

fn main() {
    println!("{}", find_min_moves(vec![1, 0, 5]));
}

#[cfg(test)]
mod tests {
    use super::find_min_moves;

    #[test]
    fn example_one() {
        assert_eq!(find_min_moves(vec![1, 0, 5]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_min_moves(vec![0, 3, 0]), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(find_min_moves(vec![0, 2, 0]), -1);
    }
}
