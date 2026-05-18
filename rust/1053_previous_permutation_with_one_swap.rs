/// LeetCode #1053 - Previous Permutation With One Swap
fn prev_perm(arr: Vec<i32>) -> Vec<i32> {
    let mut arr = arr;
    let n = arr.len();
    let mut i = n;
    for j in (0..n - 1).rev() {
        if arr[j] > arr[j + 1] {
            i = j;
            break;
        }
    }
    if i == n {
        return arr;
    }
    let mut k = n - 1;
    while arr[k] >= arr[i] {
        k -= 1;
    }
    arr.swap(i, k);
    arr[i + 1..].reverse();
    arr
}

fn main() {
    println!("{:?}", prev_perm(vec![3, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::prev_perm;

    #[test]
    fn example_one() {
        assert_eq!(prev_perm(vec![3, 2, 1]), vec![3, 1, 2]);
    }

    #[test]
    fn example_two() {
        assert_eq!(prev_perm(vec![1, 2, 3]), vec![1, 2, 3]);
    }
}
