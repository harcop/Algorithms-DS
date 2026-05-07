/// LeetCode #391 - Perfect Rectangle (coordinate compression on half-open corners)
fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
    if rectangles.is_empty() {
        return false;
    }

    let mut xs = vec![];
    let mut ys = vec![];
    let mut area_sum = 0i64;
    let mut gxmin = i32::MAX;
    let mut gymin = i32::MAX;
    let mut gxmax = i32::MIN;
    let mut gymax = i32::MIN;

    for r in &rectangles {
        let x1 = r[0];
        let y1 = r[1];
        let x2 = r[2];
        let y2 = r[3];
        gxmin = gxmin.min(x1);
        gymin = gymin.min(y1);
        gxmax = gxmax.max(x2);
        gymax = gymax.max(y2);
        area_sum += (x2 - x1) as i64 * (y2 - y1) as i64;

        xs.push(x1);
        xs.push(x2 + 1);
        ys.push(y1);
        ys.push(y2 + 1);
    }

    if area_sum != (gxmax - gxmin) as i64 * (gymax - gymin) as i64 {
        return false;
    }

    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();

    let w = xs.len() - 1;
    let h = ys.len() - 1;
    let mut grid = vec![0u8; w * h];

    for r in &rectangles {
        let x1 = r[0];
        let y1 = r[1];
        let x2 = r[2] + 1;
        let y2 = r[3] + 1;

        let ix1 = xs.binary_search(&x1).unwrap();
        let ix2 = xs.binary_search(&x2).unwrap();
        let iy1 = ys.binary_search(&y1).unwrap();
        let iy2 = ys.binary_search(&y2).unwrap();

        if ix1 >= ix2 || iy1 >= iy2 {
            return false;
        }

        for i in ix1..ix2 {
            for j in iy1..iy2 {
                let idx = i * h + j;
                if grid[idx] == 1 {
                    return false;
                }
                grid[idx] = 1;
            }
        }
    }

    grid.iter().all(|&v| v == 1)
}

fn main() {
    println!(
        "{}",
        is_rectangle_cover(vec![vec![0, 0, 4, 1], vec![7, 0, 10, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single() {
        assert!(is_rectangle_cover(vec![vec![0, 0, 1, 1]]));
        assert!(!is_rectangle_cover(vec![vec![0, 0, 1, 1], vec![1, 0, 2, 2]]));
    }
}
