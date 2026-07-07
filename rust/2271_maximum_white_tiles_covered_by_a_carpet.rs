/// LeetCode #2271 - Maximum White Tiles Covered by a Carpet
fn maximum_white_tiles(tiles: Vec<Vec<i32>>, carpet_len: i32) -> i32 {
    let mut tiles = tiles;
    tiles.sort_unstable_by_key(|t| t[0]);

    if tiles.iter().any(|t| t[1] - t[0] + 1 >= carpet_len) {
        return carpet_len;
    }

    let starts: Vec<i32> = tiles.iter().map(|t| t[0]).collect();
    let mut prefix = vec![0i32; tiles.len() + 1];
    for (i, tile) in tiles.iter().enumerate() {
        prefix[i + 1] = prefix[i] + tile[1] - tile[0] + 1;
    }

    let mut ans = 0;
    for i in 0..tiles.len() {
        let s = tiles[i][0];
        let carpet_end = s + carpet_len - 1;
        let end_index = upper_bound(&starts, carpet_end) - 1;
        let not_cover = (tiles[end_index][1] - carpet_end).max(0);
        ans = ans.max(prefix[end_index + 1] - prefix[i] - not_cover);
    }

    ans
}

fn upper_bound(arr: &[i32], target: i32) -> usize {
    let mut lo = 0usize;
    let mut hi = arr.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if arr[mid] <= target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

fn main() {
    println!(
        "{}",
        maximum_white_tiles(vec![vec![1, 5], vec![10, 11], vec![12, 18], vec![20, 25], vec![30, 32]], 10)
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_white_tiles;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_white_tiles(vec![vec![1, 5], vec![10, 11], vec![12, 18], vec![20, 25], vec![30, 32]], 10),
            9
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_white_tiles(vec![vec![10, 11], vec![1, 1]], 2), 2);
    }
}
