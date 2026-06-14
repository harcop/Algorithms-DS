/// LeetCode #1847 - Closest Room
use std::collections::BTreeSet;

fn closest_room(rooms: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut rooms = rooms;
    rooms.sort_by_key(|r| r[1]);
    let k = queries.len();
    let mut idx: Vec<usize> = (0..k).collect();
    idx.sort_by_key(|&i| queries[i][1]);

    let mut ans = vec![-1; k];
    let mut i = 0usize;
    let n = rooms.len();
    let mut set: BTreeSet<i32> = rooms.iter().map(|r| r[0]).collect();

    for &j in &idx {
        let prefer = queries[j][0];
        let min_size = queries[j][1];
        while i < n && rooms[i][1] < min_size {
            set.remove(&rooms[i][0]);
            i += 1;
        }
        if i == n {
            break;
        }

        let mut candidate = -1i32;
        if let Some(&ge) = set.range(prefer..).next() {
            candidate = ge;
        }
        if let Some(&le) = set.range(..=prefer).next_back() {
            if candidate == -1 || candidate - prefer >= prefer - le {
                candidate = le;
            }
        }
        ans[j] = candidate;
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        closest_room(
            vec![vec![2, 2], vec![1, 2], vec![3, 2]],
            vec![vec![3, 1], vec![3, 3], vec![5, 2]],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::closest_room;

    #[test]
    fn example_one() {
        assert_eq!(
            closest_room(
                vec![vec![2, 2], vec![1, 2], vec![3, 2]],
                vec![vec![3, 1], vec![3, 3], vec![5, 2]],
            ),
            vec![3, -1, 3]
        );
    }
}
