/// LeetCode #646 - Maximum Length of Pair Chain
fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
    pairs.sort_unstable_by_key(|p| p[1]);
    let mut count = 0i32;
    let mut cur = i32::MIN;
    for p in pairs {
        if p[0] > cur {
            count += 1;
            cur = p[1];
        }
    }
    count
}

fn main() {
    println!("{}", find_longest_chain(vec![vec![1, 2], vec![2, 3], vec![3, 4]]));
}

#[cfg(test)]
mod tests {
    use super::find_longest_chain;

    #[test]
    fn example_one() {
        assert_eq!(find_longest_chain(vec![vec![1, 2], vec![2, 3], vec![3, 4]]), 2);
    }
}
