/// LeetCode #1146 - Snapshot Array
struct SnapshotArray {
    history: Vec<Vec<(i32, i32)>>,
    snap_id: i32,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        SnapshotArray {
            history: vec![Vec::new(); length as usize],
            snap_id: 0,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        let h = &mut self.history[index as usize];
        if h.is_empty() || h.last().unwrap().0 != self.snap_id {
            h.push((self.snap_id, val));
        } else {
            h.last_mut().unwrap().1 = val;
        }
    }

    fn snap(&mut self) -> i32 {
        let id = self.snap_id;
        self.snap_id += 1;
        id
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let h = &self.history[index as usize];
        let mut lo = 0usize;
        let mut hi = h.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if h[mid].0 <= snap_id {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        if lo == 0 { 0 } else { h[lo - 1].1 }
    }
}

fn main() {
    let mut sa = SnapshotArray::new(3);
    sa.set(0, 5);
    let _ = sa.snap();
    sa.set(0, 6);
    println!("{}", sa.get(0, 0));
}

#[cfg(test)]
mod tests {
    use super::SnapshotArray;

    #[test]
    fn example_one() {
        let mut sa = SnapshotArray::new(3);
        sa.set(0, 5);
        assert_eq!(sa.snap(), 0);
        sa.set(0, 6);
        assert_eq!(sa.get(0, 0), 5);
        assert_eq!(sa.get(0, 1), 6);
    }
}
