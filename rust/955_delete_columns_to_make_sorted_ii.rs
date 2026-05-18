/// LeetCode #955 - Delete Columns to Make Sorted II

fn min_deletion_size(strs: Vec<String>) -> i32 {
    if strs.is_empty() {
        return 0;
    }
    let rows = strs.len();
    let cols = strs[0].len();
    let mut deleted = 0usize;
    let mut done = vec![false; rows - 1];
    for c in 0..cols {
        let mut remove = false;
        for i in 0..rows - 1 {
            if done[i] {
                continue;
            }
            let a = strs[i].as_bytes()[c];
            let b = strs[i + 1].as_bytes()[c];
            if a > b {
                remove = true;
                break;
            }
        }
        if remove {
            deleted += 1;
            continue;
        }
        for i in 0..rows - 1 {
            if done[i] {
                continue;
            }
            let a = strs[i].as_bytes()[c];
            let b = strs[i + 1].as_bytes()[c];
            if a < b {
                done[i] = true;
            }
        }
    }
    deleted as i32
}

fn main() {
    println!("{}", min_deletion_size(vec!["ca".into(), "bb".into(), "ac".into()]));
}

#[cfg(test)]
mod tests {
    use super::min_deletion_size;

    #[test]
    fn example_one() {
        assert_eq!(min_deletion_size(vec!["ca".into(), "bb".into(), "ac".into()]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_deletion_size(vec!["xc".into(), "yb".into(), "za".into()]), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(
            min_deletion_size(vec!["zyx".into(), "wvu".into(), "tsr".into()]),
            3
        );
    }
}
