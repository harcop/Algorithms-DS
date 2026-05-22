/// LeetCode #1229 - Meeting Scheduler
fn min_available_duration(slots1: Vec<Vec<i32>>, slots2: Vec<Vec<i32>>, duration: i32) -> Vec<i32> {
    let mut a: Vec<(i32, i32)> = slots1.into_iter().map(|v| (v[0], v[1])).collect();
    let mut b: Vec<(i32, i32)> = slots2.into_iter().map(|v| (v[0], v[1])).collect();
    a.sort_unstable();
    b.sort_unstable();
    let mut i = 0usize;
    let mut j = 0usize;
    while i < a.len() && j < b.len() {
        let start = a[i].0.max(b[j].0);
        let end = a[i].1.min(b[j].1);
        if end - start >= duration {
            return vec![start, start + duration];
        }
        if a[i].1 < b[j].1 {
            i += 1;
        } else {
            j += 1;
        }
    }
    vec![]
}

fn main() {
    println!(
        "{:?}",
        min_available_duration(vec![vec![10, 50], vec![60, 120], vec![140, 210]], vec![vec![0, 15], vec![60, 70]], 8)
    );
}

#[cfg(test)]
mod tests {
    use super::min_available_duration;

    #[test]
    fn example_one() {
        assert_eq!(
            min_available_duration(
                vec![vec![10, 50], vec![60, 120], vec![140, 210]],
                vec![vec![0, 15], vec![60, 70]],
                8
            ),
            vec![60, 68]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            min_available_duration(vec![vec![10, 50], vec![60, 120], vec![140, 210]], vec![vec![0, 15], vec![60, 70]], 12),
            vec![]
        );
    }
}
