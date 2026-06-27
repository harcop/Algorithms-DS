/// LeetCode #2111 - Minimum Operations to Make the Array K-Increasing
fn k_increasing(arr: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut ans = 0i32;

    for start in 0..k {
        let mut tails = Vec::new();
        let mut len = 0i32;
        let mut i = start;

        while i < arr.len() {
            len += 1;
            let pos = tails.partition_point(|&x| x <= arr[i]);
            if pos == tails.len() {
                tails.push(arr[i]);
            } else {
                tails[pos] = arr[i];
            }
            i += k;
        }

        ans += len - tails.len() as i32;
    }

    ans
}

fn main() {
    println!("{}", k_increasing(vec![5, 4, 3, 2, 1], 1));
}

#[cfg(test)]
mod tests {
    use super::k_increasing;

    #[test]
    fn example_one() {
        assert_eq!(k_increasing(vec![5, 4, 3, 2, 1], 1), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(k_increasing(vec![4, 1, 5, 2, 6, 2], 2), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(k_increasing(vec![4, 1, 5, 2, 6, 2], 3), 2);
    }
}
