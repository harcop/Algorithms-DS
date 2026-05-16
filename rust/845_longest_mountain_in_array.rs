/// LeetCode #845 - Longest Mountain in Array
fn longest_mountain(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut ans = 0;
    let mut i = 0;
    while i + 1 < n {
        while i + 1 < n && arr[i] == arr[i + 1] {
            i += 1;
        }
        let mut up = 0;
        while i + 1 < n && arr[i] < arr[i + 1] {
            up += 1;
            i += 1;
        }
        let mut down = 0;
        while i + 1 < n && arr[i] > arr[i + 1] {
            down += 1;
            i += 1;
        }
        if up > 0 && down > 0 {
            ans = ans.max(up + down + 1);
        }
        if down == 0 {
            i += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", longest_mountain(vec![2, 1, 4, 7, 3, 2, 5]));
}

#[cfg(test)]
mod tests {
    use super::longest_mountain;

    #[test]
    fn example_one() {
        assert_eq!(longest_mountain(vec![2, 1, 4, 7, 3, 2, 5]), 5);
    }
}
