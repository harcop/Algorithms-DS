/// LeetCode #1199 - Minimum Time to Build Blocks
fn min_build_time(blocks: Vec<i32>, split: i32) -> i32 {
    let mut blocks = blocks;
    blocks.sort_unstable();
    let mut split = split as usize;
    while blocks.len() > 1 && split > 0 {
        let a = blocks.remove(0);
        let b = blocks.remove(0);
        let merged = a + 2 * b;
        let pos = blocks.binary_search(&merged).unwrap_or_else(|p| p);
        blocks.insert(pos, merged);
        split -= 1;
    }
    *blocks.last().unwrap()
}

fn main() {
    println!("{}", min_build_time(vec![1], 3));
}

#[cfg(test)]
mod tests {
    use super::min_build_time;

    #[test]
    fn example_one() {
        assert_eq!(min_build_time(vec![1], 3), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_build_time(vec![1, 2, 3, 4, 5, 6, 7, 8], 3), 15);
    }
}
