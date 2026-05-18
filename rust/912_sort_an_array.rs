/// LeetCode #912 - Sort an Array
fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
    merge_sort(&mut nums);
    nums
}

fn merge_sort(a: &mut [i32]) {
    let n = a.len();
    if n <= 1 {
        return;
    }
    let mid = n / 2;
    merge_sort(&mut a[..mid]);
    merge_sort(&mut a[mid..]);
    let mut tmp = Vec::with_capacity(n);
    let (l, r) = a.split_at(mid);
    let mut i = 0usize;
    let mut j = 0usize;
    while i < l.len() && j < r.len() {
        if l[i] <= r[j] {
            tmp.push(l[i]);
            i += 1;
        } else {
            tmp.push(r[j]);
            j += 1;
        }
    }
    tmp.extend_from_slice(&l[i..]);
    tmp.extend_from_slice(&r[j..]);
    a.copy_from_slice(&tmp);
}

fn main() {
    println!("{:?}", sort_array(vec![5, 2, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::sort_array;

    #[test]
    fn example_one() {
        assert_eq!(sort_array(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
    }

    #[test]
    fn example_two() {
        assert_eq!(sort_array(vec![5, 1, 1, 2, 0, 0]), vec![0, 0, 1, 1, 2, 5]);
    }
}
