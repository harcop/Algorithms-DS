/// LeetCode #3951 - Minimum Energy to Maintain Brightness
fn min_energy(_n: i32, brightness: i32, mut intervals: Vec<Vec<i32>>) -> i64 {
    intervals.sort_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));
    let mut merged: Vec<(i64, i64)> = vec![(intervals[0][0] as i64, intervals[0][1] as i64)];
    for x in intervals.iter().skip(1) {
        let start = x[0] as i64;
        let end = x[1] as i64;
        if merged.last().unwrap().1 < start {
            merged.push((start, end));
        } else {
            let last = merged.last_mut().unwrap();
            last.1 = last.1.max(end);
        }
    }
    let bulbs = (brightness as i64 + 2) / 3;
    merged.iter().map(|(s, e)| bulbs * (e - s + 1)).sum()
}

fn main() {
    println!("{}", min_energy(5, 5, vec![vec![6, 12]]));
}

#[cfg(test)]
mod tests {
    use super::min_energy;

    #[test]
    fn example1() {
        assert_eq!(min_energy(5, 5, vec![vec![6, 12]]), 14);
    }

    #[test]
    fn example2() {
        assert_eq!(min_energy(2, 1, vec![vec![0, 0], vec![2, 2]]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(min_energy(4, 2, vec![vec![1, 3], vec![2, 4]]), 4);
    }
}
