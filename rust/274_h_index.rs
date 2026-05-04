/// LeetCode #274 - H-Index
fn h_index(mut citations: Vec<i32>) -> i32 {
    citations.sort_unstable_by(|a, b| b.cmp(a));
    let mut h = 0;
    for (i, &c) in citations.iter().enumerate() {
        if c >= (i + 1) as i32 {
            h = (i + 1) as i32;
        } else {
            break;
        }
    }
    h
}

fn main() {
    println!("{}", h_index(vec![3, 0, 6, 1, 5]));
}

#[cfg(test)]
mod tests {
    use super::h_index;

    #[test]
    fn example_one() {
        assert_eq!(h_index(vec![3, 0, 6, 1, 5]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(h_index(vec![1, 3, 1]), 1);
    }
}
