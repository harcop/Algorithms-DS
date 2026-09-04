/// LeetCode #1810 - Minimum Path Cost in a Hidden Grid
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

struct GridMaster {
    grid: Vec<Vec<i32>>,
    r: usize,
    c: usize,
    tr: usize,
    tc: usize,
}

impl GridMaster {
    fn new(grid: Vec<Vec<i32>>, sr: usize, sc: usize, tr: usize, tc: usize) -> Self {
        GridMaster {
            grid,
            r: sr,
            c: sc,
            tr,
            tc,
        }
    }

    fn in_bounds(&self, r: isize, c: isize) -> bool {
        r >= 0 && c >= 0 && (r as usize) < self.grid.len() && (c as usize) < self.grid[0].len()
    }

    fn can_move(&self, direction: char) -> bool {
        let (dr, dc) = delta(direction);
        let nr = self.r as isize + dr;
        let nc = self.c as isize + dc;
        self.in_bounds(nr, nc) && self.grid[nr as usize][nc as usize] > 0
    }

    fn move_dir(&mut self, direction: char) -> i32 {
        if !self.can_move(direction) {
            return -1;
        }
        let (dr, dc) = delta(direction);
        self.r = (self.r as isize + dr) as usize;
        self.c = (self.c as isize + dc) as usize;
        self.grid[self.r][self.c]
    }

    fn is_target(&self) -> bool {
        self.r == self.tr && self.c == self.tc
    }
}

fn delta(d: char) -> (isize, isize) {
    match d {
        'U' => (-1, 0),
        'D' => (1, 0),
        'L' => (0, -1),
        'R' => (0, 1),
        _ => (0, 0),
    }
}

fn opposite(d: char) -> char {
    match d {
        'U' => 'D',
        'D' => 'U',
        'L' => 'R',
        'R' => 'L',
        _ => d,
    }
}

fn find_shortest_path(master: &mut GridMaster) -> i32 {
    let mut costs: HashMap<(i32, i32), i32> = HashMap::new();
    let mut target: Option<(i32, i32)> = None;
    costs.insert((0, 0), 0);
    if master.is_target() {
        target = Some((0, 0));
    }
    explore(master, 0, 0, &mut costs, &mut target);
    let Some(t) = target else {
        return -1;
    };
    dijkstra(&costs, t)
}

fn explore(
    master: &mut GridMaster,
    r: i32,
    c: i32,
    costs: &mut HashMap<(i32, i32), i32>,
    target: &mut Option<(i32, i32)>,
) {
    for d in ['U', 'D', 'L', 'R'] {
        if !master.can_move(d) {
            continue;
        }
        let (dr, dc) = delta(d);
        let nr = r + dr as i32;
        let nc = c + dc as i32;
        if costs.contains_key(&(nr, nc)) {
            continue;
        }
        let cost = master.move_dir(d);
        costs.insert((nr, nc), cost);
        if master.is_target() {
            *target = Some((nr, nc));
        }
        explore(master, nr, nc, costs, target);
        master.move_dir(opposite(d));
    }
}

fn dijkstra(costs: &HashMap<(i32, i32), i32>, target: (i32, i32)) -> i32 {
    let mut dist: HashMap<(i32, i32), i32> = HashMap::new();
    let mut pq: BinaryHeap<Reverse<(i32, i32, i32)>> = BinaryHeap::new();
    dist.insert((0, 0), 0);
    pq.push(Reverse((0, 0, 0)));
    let mut seen = HashSet::new();
    while let Some(Reverse((d, r, c))) = pq.pop() {
        if !seen.insert((r, c)) {
            continue;
        }
        if (r, c) == target {
            return d;
        }
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nr = r + dr;
            let nc = c + dc;
            if let Some(&cell) = costs.get(&(nr, nc)) {
                let nd = d + cell;
                if dist.get(&(nr, nc)).map_or(true, |&old| nd < old) {
                    dist.insert((nr, nc), nd);
                    pq.push(Reverse((nd, nr, nc)));
                }
            }
        }
    }
    -1
}

fn main() {
    let grid = vec![vec![1, 2], vec![3, 4]];
    let mut m = GridMaster::new(grid, 0, 0, 1, 1);
    println!("{}", find_shortest_path(&mut m));
}

#[cfg(test)]
mod tests {
    use super::{find_shortest_path, GridMaster};

    #[test]
    fn example_one() {
        let grid = vec![vec![1, 2], vec![3, 4]];
        let mut m = GridMaster::new(grid, 0, 0, 1, 1);
        assert_eq!(find_shortest_path(&mut m), 6);
    }

    #[test]
    fn blocked() {
        let grid = vec![vec![1, 0], vec![0, 4]];
        let mut m = GridMaster::new(grid, 0, 0, 1, 1);
        assert_eq!(find_shortest_path(&mut m), -1);
    }
}
