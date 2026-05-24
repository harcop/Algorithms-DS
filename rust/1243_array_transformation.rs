/// LeetCode #1243 - Array Transformation
fn transform_array(arr: Vec<i32>) -> Vec<i32> {
    let mut a = arr;
    loop {
        let mut next = a.clone();
        let mut changed = false;
        for i in 1..a.len() - 1 {
            if a[i - 1] > a[i] && a[i] < a[i + 1] {
                next[i] = a[i] * 2;
                changed = true;
            } else if a[i - 1] < a[i] && a[i] > a[i + 1] {
                next[i] = a[i] / 2;
                changed = true;
            }
        }
        a = next;
        if !changed {
            break;
        }
    }
    a
}

fn main() {
    println!("{:?}", transform_array(vec![6, 2, 3, 8]));
}

#[cfg(test)]
mod tests {
    use super::transform_array;

    #[test]
    fn example_one() {
        assert_eq!(transform_array(vec![6, 2, 3, 8]), vec![6, 4, 6, 8]);
    }

    #[test]
    fn example_two() {
        assert_eq!(transform_array(vec![1, 2]), vec![1, 2]);
    }
}
