/// LeetCode #2426 - Number of Pairs Satisfying Inequality
fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, diff: i32) -> i64 {
    fn sort_and_count(values: &mut [i64], diff: i64, buffer: &mut [i64]) -> i64 {
        let n = values.len();
        if n <= 1 {
            return 0;
        }

        let mid = n / 2;
        let (left, right) = values.split_at_mut(mid);
        let (left_buffer, right_buffer) = buffer.split_at_mut(mid);
        let mut answer =
            sort_and_count(left, diff, left_buffer) + sort_and_count(right, diff, right_buffer);

        let mut j = 0;
        for &value in left.iter() {
            while j < right.len() && value > right[j] + diff {
                j += 1;
            }
            answer += (right.len() - j) as i64;
        }

        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                buffer[k] = left[i];
                i += 1;
            } else {
                buffer[k] = right[j];
                j += 1;
            }
            k += 1;
        }
        buffer[k..k + left.len() - i].copy_from_slice(&left[i..]);
        k += left.len() - i;
        buffer[k..k + right.len() - j].copy_from_slice(&right[j..]);
        values.copy_from_slice(buffer);

        answer
    }

    let mut values: Vec<i64> = nums1
        .into_iter()
        .zip(nums2)
        .map(|(a, b)| (a - b) as i64)
        .collect();
    let mut buffer = vec![0; values.len()];
    sort_and_count(&mut values, diff as i64, &mut buffer)
}

fn main() {
    println!("{}", number_of_pairs(vec![3, 2, 5], vec![2, 2, 1], 1));
}

#[cfg(test)]
mod tests {
    use super::number_of_pairs;

    #[test]
    fn example_one() {
        assert_eq!(number_of_pairs(vec![3, 2, 5], vec![2, 2, 1], 1), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_pairs(vec![3, -1], vec![-2, 2], -1), 0);
    }
}
