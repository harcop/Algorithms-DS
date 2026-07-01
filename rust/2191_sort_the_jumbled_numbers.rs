/// LeetCode #2191 - Sort the Jumbled Numbers
fn mapped_value(x: i32, mapping: &[i32]) -> i32 {
    if x == 0 {
        return mapping[0];
    }
    let mut y = 0i32;
    let mut k = 1i32;
    let mut num = x;
    while num != 0 {
        let v = mapping[(num % 10) as usize];
        y = k * v + y;
        k *= 10;
        num /= 10;
    }
    y
}

fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
    let mut indexed: Vec<(i32, usize)> = nums
        .iter()
        .enumerate()
        .map(|(i, &x)| (mapped_value(x, &mapping), i))
        .collect();
    indexed.sort_by_key(|&(v, i)| (v, i));
    indexed.into_iter().map(|(_, i)| nums[i]).collect()
}

fn main() {
    println!(
        "{:?}",
        sort_jumbled(vec![8, 9, 4, 0, 2, 1, 3, 5, 7, 6], vec![991, 338, 38])
    );
}

#[cfg(test)]
mod tests {
    use super::sort_jumbled;

    #[test]
    fn example_one() {
        assert_eq!(
            sort_jumbled(vec![8, 9, 4, 0, 2, 1, 3, 5, 7, 6], vec![991, 338, 38]),
            vec![338, 38, 991]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            sort_jumbled(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec![789, 456, 123]),
            vec![123, 456, 789]
        );
    }
}
