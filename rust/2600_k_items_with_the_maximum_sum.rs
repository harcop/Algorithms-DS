/// LeetCode #2600 - K Items With the Maximum Sum
fn k_items_with_maximum_sum(
    num_ones: i32,
    num_zeros: i32,
    _num_neg_ones: i32,
    k: i32,
) -> i32 {
    if num_ones >= k {
        return k;
    }
    if num_zeros >= k - num_ones {
        return num_ones;
    }
    num_ones - (k - num_ones - num_zeros)
}

fn main() {
    println!("{}", k_items_with_maximum_sum(3, 2, 0, 2));
}

#[cfg(test)]
mod tests {
    use super::k_items_with_maximum_sum;

    #[test]
    fn example_one() {
        assert_eq!(k_items_with_maximum_sum(3, 2, 0, 2), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(k_items_with_maximum_sum(3, 2, 0, 4), 3);
    }
}
