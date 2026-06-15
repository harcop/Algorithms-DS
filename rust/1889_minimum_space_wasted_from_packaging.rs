/// LeetCode #1889 - Minimum Space Wasted From Packaging
const MOD: i64 = 1_000_000_007;
const INF: i64 = i64::MAX / 2;

fn min_wasted_space(mut packages: Vec<i32>, mut boxes: Vec<Vec<i32>>) -> i32 {
    packages.sort_unstable();
    let total: i64 = packages.iter().map(|&x| x as i64).sum();
    let mut ans = INF;
    for box_list in &mut boxes {
        box_list.sort_unstable();
        if packages[packages.len() - 1] > box_list[box_list.len() - 1] {
            continue;
        }
        let mut s = 0i64;
        let mut i = 0usize;
        for &b in box_list.iter() {
            let j = packages.partition_point(|&x| x <= b).max(i);
            s += (j - i) as i64 * b as i64;
            i = j;
        }
        ans = ans.min(s);
    }
    if ans == INF {
        -1
    } else {
        ((ans - total) % MOD) as i32
    }
}

fn main() {
    println!(
        "{}",
        min_wasted_space(vec![2, 3, 5], vec![vec![4, 8], vec![2, 8]])
    );
}

#[cfg(test)]
mod tests {
    use super::min_wasted_space;

    #[test]
    fn example_one() {
        assert_eq!(
            min_wasted_space(vec![2, 3, 5], vec![vec![4, 8], vec![2, 8]]),
            6
        );
    }
}
