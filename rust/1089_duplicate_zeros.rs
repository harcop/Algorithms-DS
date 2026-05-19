/// LeetCode #1089 - Duplicate Zeros
fn duplicate_zeros(arr: &mut Vec<i32>) {
    let mut zeros = 0;
    for &x in arr.iter() {
        if x == 0 {
            zeros += 1;
        }
    }
    let mut k = arr.len() + zeros;
    let n = arr.len();
    let mut i = n;
    let mut j = k;
    while i > 0 {
        i -= 1;
        j -= 1;
        if j < n {
            arr[j] = arr[i];
        }
        if arr[i] == 0 {
            j -= 1;
            if j < n {
                arr[j] = 0;
            }
        }
    }
}

fn main() {
    let mut v = vec![1, 0, 2, 3, 0, 4, 5, 0];
    duplicate_zeros(&mut v);
    println!("{:?}", v);
}

#[cfg(test)]
mod tests {
    use super::duplicate_zeros;

    #[test]
    fn example_one() {
        let mut v = vec![1, 0, 2, 3, 0, 4, 5, 0];
        duplicate_zeros(&mut v);
        assert_eq!(v, vec![1, 0, 0, 2, 3, 0, 0, 4]);
    }

    #[test]
    fn example_two() {
        let mut v = vec![1, 2, 3];
        duplicate_zeros(&mut v);
        assert_eq!(v, vec![1, 2, 3]);
    }
}
