/// LeetCode #733 - Flood Fill
fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    let m = image.len();
    let n = image[0].len();
    let sr = sr as usize;
    let sc = sc as usize;
    let start = image[sr][sc];
    if start == color {
        return image;
    }
    let mut stack = vec![(sr, sc)];
    while let Some((r, c)) = stack.pop() {
        if image[r][c] != start {
            continue;
        }
        image[r][c] = color;
        if r > 0 && image[r - 1][c] == start {
            stack.push((r - 1, c));
        }
        if r + 1 < m && image[r + 1][c] == start {
            stack.push((r + 1, c));
        }
        if c > 0 && image[r][c - 1] == start {
            stack.push((r, c - 1));
        }
        if c + 1 < n && image[r][c + 1] == start {
            stack.push((r, c + 1));
        }
    }
    image
}

fn main() {
    let g = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
    println!("{:?}", flood_fill(g, 1, 1, 2));
}

#[cfg(test)]
mod tests {
    use super::flood_fill;

    #[test]
    fn example_one() {
        let g = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let e = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
        assert_eq!(flood_fill(g, 1, 1, 2), e);
    }
}
