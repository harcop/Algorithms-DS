/// LeetCode #927 - Three Equal Parts
fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
    let n = arr.len();
    let cnt: i32 = arr.iter().sum();
    if cnt % 3 != 0 {
        return vec![-1, -1];
    }
    if cnt == 0 {
        return vec![0, (n - 1) as i32];
    }
    let third = (cnt / 3) as i32;

    let find = |target: i32| -> usize {
        let mut s = 0;
        for (i, &v) in arr.iter().enumerate() {
            s += v;
            if s == target {
                return i;
            }
        }
        0
    };

    let mut i = find(1);
    let mut j = find(third + 1);
    let mut k = find(2 * third + 1);
    while k < n && arr[i] == arr[j] && arr[j] == arr[k] {
        i += 1;
        j += 1;
        k += 1;
    }
    if k == n {
        vec![(i - 1) as i32, j as i32]
    } else {
        vec![-1, -1]
    }
}

fn main() {
    println!("{:?}", three_equal_parts(vec![1, 0, 1, 0, 1]));
}

#[cfg(test)]
mod tests {
    use super::three_equal_parts;

    #[test]
    fn example_one() {
        assert_eq!(three_equal_parts(vec![1, 0, 1, 0, 1]), vec![0, 3]);
    }

    #[test]
    fn example_two() {
        assert_eq!(three_equal_parts(vec![1, 1, 0, 1, 1]), vec![-1, -1]);
    }

    #[test]
    fn example_three() {
        assert_eq!(three_equal_parts(vec![1, 1, 0, 0, 1]), vec![0, 2]);
    }
}
