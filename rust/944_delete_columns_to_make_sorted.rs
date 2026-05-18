/// LeetCode #944 - Delete Columns to Make Sorted

fn min_deletion_size(strs: Vec<String>) -> i32 {
    if strs.is_empty() {
        return 0;
    }
    let cols = strs[0].len();
    let mut deleted = 0;
    for c in 0..cols {
        let mut ok = true;
        for i in 1..strs.len() {
            let a = strs[i - 1].as_bytes()[c];
            let b = strs[i].as_bytes()[c];
            if a > b {
                ok = false;
                break;
            }
        }
        if !ok {
            deleted += 1;
        }
    }
    deleted
}

fn main() {
    println!("{}", min_deletion_size(vec!["cba".into(), "daf".into(), "ghi".into()]));
}

#[cfg(test)]
mod tests {
    use super::min_deletion_size;

    #[test]
    fn example_one() {
        assert_eq!(min_deletion_size(vec!["cba".into(), "daf".into(), "ghi".into()]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_deletion_size(vec!["a".into(), "b".into()]), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(
            min_deletion_size(vec!["zyx".into(), "wvu".into(), "tsr".into()]),
            3
        );
    }
}
