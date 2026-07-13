/// LeetCode #2379 - Minimum Recolors to Get K Consecutive Black Blocks
fn minimum_recolors(blocks: String, k: i32) -> i32 {
    let b = blocks.as_bytes();
    let k = k as usize;
    let mut white = b[..k].iter().filter(|&&c| c == b'W').count() as i32;
    let mut ans = white;

    for i in k..b.len() {
        if b[i] == b'W' {
            white += 1;
        }
        if b[i - k] == b'W' {
            white -= 1;
        }
        ans = ans.min(white);
    }

    ans
}

fn main() {
    println!("{}", minimum_recolors("WBBWWBBWBW".to_string(), 7));
}

#[cfg(test)]
mod tests {
    use super::minimum_recolors;

    #[test]
    fn example_one() {
        assert_eq!(minimum_recolors("WBBWWBBWBW".to_string(), 7), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_recolors("WBWBBBW".to_string(), 2), 0);
    }
}
