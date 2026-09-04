/// LeetCode #1274 - Number of Ships in a Rectangle
struct Sea {
    ships: Vec<(i32, i32)>,
}

impl Sea {
    fn new(ships: Vec<(i32, i32)>) -> Self {
        Sea { ships }
    }

    fn has_ships(&self, top_right: (i32, i32), bottom_left: (i32, i32)) -> bool {
        let (x2, y2) = top_right;
        let (x1, y1) = bottom_left;
        if x1 > x2 || y1 > y2 {
            return false;
        }
        self.ships
            .iter()
            .any(|&(x, y)| x >= x1 && x <= x2 && y >= y1 && y <= y2)
    }
}

fn count_ships(sea: &Sea, top_right: (i32, i32), bottom_left: (i32, i32)) -> i32 {
    let (x2, y2) = top_right;
    let (x1, y1) = bottom_left;
    if x1 > x2 || y1 > y2 || !sea.has_ships(top_right, bottom_left) {
        return 0;
    }
    if x1 == x2 && y1 == y2 {
        return 1;
    }
    let mx = (x1 + x2) / 2;
    let my = (y1 + y2) / 2;
    count_ships(sea, (mx, my), (x1, y1))
        + count_ships(sea, (x2, y2), (mx + 1, my + 1))
        + count_ships(sea, (mx, y2), (x1, my + 1))
        + count_ships(sea, (x2, my), (mx + 1, y1))
}

fn main() {
    let sea = Sea::new(vec![(1, 1), (2, 2), (3, 3)]);
    println!("{}", count_ships(&sea, (4, 4), (0, 0)));
}

#[cfg(test)]
mod tests {
    use super::{count_ships, Sea};

    #[test]
    fn example_one() {
        let sea = Sea::new(vec![(1, 1), (2, 2), (3, 3)]);
        assert_eq!(count_ships(&sea, (4, 4), (0, 0)), 3);
    }

    #[test]
    fn example_two() {
        let sea = Sea::new(vec![(1, 1), (2, 2), (3, 3)]);
        assert_eq!(count_ships(&sea, (1000, 1000), (0, 0)), 3);
    }
}
