/// LeetCode #1079 - Letter Tile Possibilities
fn num_tile_possibilities(tiles: String) -> i32 {
    let mut cnt = [0i32; 26];
    for c in tiles.bytes() {
        cnt[(c - b'A') as usize] += 1;
    }
    fn dfs(cnt: &mut [i32; 26]) -> i32 {
        let mut total = 0i32;
        for i in 0..26 {
            if cnt[i] == 0 {
                continue;
            }
            cnt[i] -= 1;
            total += 1 + dfs(cnt);
            cnt[i] += 1;
        }
        total
    }
    dfs(&mut cnt)
}

fn main() {
    println!("{}", num_tile_possibilities("AAB".into()));
}

#[cfg(test)]
mod tests {
    use super::num_tile_possibilities;

    #[test]
    fn example_one() {
        assert_eq!(num_tile_possibilities("AAB".into()), 8);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_tile_possibilities("AAABBC".into()), 188);
    }
}
