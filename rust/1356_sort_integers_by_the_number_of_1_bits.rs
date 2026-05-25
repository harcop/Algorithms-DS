/// LeetCode #1356 - Sort Integers By The Number Of 1 Bits

fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
    let mut v = arr;
    v.sort_by(|&a, &b| {
        let ca = a.count_ones();
        let cb = b.count_ones();
        ca.cmp(&cb).then_with(|| a.cmp(&b))
    });
    v
}

fn main() {
    println!("{:?}", sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
}

#[cfg(test)]
mod tests {
    use super::sort_by_bits;

    #[test]
    fn example_one() {
        assert_eq!(sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), vec![0, 1, 2, 4, 8, 3, 5, 6, 9, 10, 7]);
    }
}
