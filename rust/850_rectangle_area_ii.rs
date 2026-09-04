/// LeetCode #850 - Rectangle Area II
fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut events: Vec<(i32, i32, i32, i32)> = Vec::new();
    for r in &rectangles {
        events.push((r[0], r[1], r[3], 1));
        events.push((r[2], r[1], r[3], -1));
    }
    events.sort_unstable();
    let mut open: Vec<(i32, i32)> = Vec::new();
    let mut area: i64 = 0;
    let mut prev_x = events[0].0;
    let mut i = 0;
    while i < events.len() {
        let x = events[i].0;
        area += (x as i64 - prev_x as i64) * covered_y(&open);
        while i < events.len() && events[i].0 == x {
            let (_, y1, y2, d) = events[i];
            if d == 1 {
                open.push((y1, y2));
            } else if let Some(pos) = open.iter().position(|&iv| iv == (y1, y2)) {
                open.swap_remove(pos);
            }
            i += 1;
        }
        prev_x = x;
    }
    (area % MOD) as i32
}

fn covered_y(open: &[(i32, i32)]) -> i64 {
    if open.is_empty() {
        return 0;
    }
    let mut ivs = open.to_vec();
    ivs.sort_unstable();
    let mut total = 0i64;
    let mut cur_s = ivs[0].0;
    let mut cur_e = ivs[0].1;
    for &(s, e) in ivs.iter().skip(1) {
        if s > cur_e {
            total += (cur_e - cur_s) as i64;
            cur_s = s;
            cur_e = e;
        } else {
            cur_e = cur_e.max(e);
        }
    }
    total += (cur_e - cur_s) as i64;
    total
}

fn main() {
    println!(
        "{}",
        rectangle_area(vec![vec![0, 0, 2, 2], vec![1, 0, 2, 3], vec![1, 0, 3, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::rectangle_area;

    #[test]
    fn example_one() {
        assert_eq!(
            rectangle_area(vec![vec![0, 0, 2, 2], vec![1, 0, 2, 3], vec![1, 0, 3, 1]]),
            6
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            rectangle_area(vec![vec![0, 0, 1_000_000_000, 1_000_000_000]]),
            49
        );
    }
}
