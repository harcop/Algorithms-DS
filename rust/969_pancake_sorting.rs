/// LeetCode #969 - Pancake Sorting
fn pancake_sort(arr: &mut Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::new();
    let mut n = arr.len();
    while n > 1 {
        let mut k = 0;
        for i in 1..n {
            if arr[i] > arr[k] {
                k = i;
            }
        }
        if k == n - 1 {
            n -= 1;
            continue;
        }
        if k > 0 {
            arr[..=k].reverse();
            ans.push((k + 1) as i32);
        }
        arr[..n].reverse();
        ans.push(n as i32);
        n -= 1;
    }
    ans
}

fn main() {
    let mut v = vec![3, 2, 4, 1];
    println!("{:?}", pancake_sort(&mut v));
}

#[cfg(test)]
mod tests {
    use super::pancake_sort;

    #[test]
    fn example_one() {
        let mut v = vec![3, 2, 4, 1];
        pancake_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    #[test]
    fn example_two() {
        let mut v = vec![1, 2, 3];
        assert_eq!(pancake_sort(&mut v), Vec::<i32>::new());
    }
}
