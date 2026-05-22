/// LeetCode #1213 - Intersection of Three Sorted Arrays
fn arrays_intersection(arr1: Vec<i32>, arr2: Vec<i32>, arr3: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let mut ans = Vec::new();
    while i < arr1.len() && j < arr2.len() && k < arr3.len() {
        let a = arr1[i];
        let b = arr2[j];
        let c = arr3[k];
        let m = a.min(b).min(c);
        if a == b && b == c {
            if ans.last().copied() != Some(m) {
                ans.push(m);
            }
        }
        if a <= m {
            i += 1;
        }
        if b <= m {
            j += 1;
        }
        if c <= m {
            k += 1;
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        arrays_intersection(vec![1, 2, 3], vec![3, 4, 5], vec![3, 5, 6])
    );
}

#[cfg(test)]
mod tests {
    use super::arrays_intersection;

    #[test]
    fn example_one() {
        assert_eq!(
            arrays_intersection(vec![1, 2, 3], vec![3, 4, 5], vec![3, 5, 6]),
            vec![3]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            arrays_intersection(vec![1, 2, 3, 4, 5], vec![1, 2, 5, 7, 9], vec![1, 3, 4, 5, 8, 9]),
            vec![1, 5]
        );
    }
}
