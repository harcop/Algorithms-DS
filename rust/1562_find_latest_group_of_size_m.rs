/// LeetCode #1562 - Find Latest Group Of Size M
fn find_latest_step(arr: Vec<i32>, m: i32) -> i32 {
    if m == 0 {
        return 0;
    }
    let n = arr.len();
    let mut parent: Vec<usize> = (0..n).collect();
    let mut size = vec![0usize; n];
    let mut ans = -1;
    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }
    for (step, &v) in arr.iter().enumerate() {
        let i = (v - 1) as usize;
        size[i] = 1;
        for nb in [i.wrapping_sub(1), i + 1] {
            if nb < n && size[nb] > 0 {
                let a = find(&mut parent, i);
                let b = find(&mut parent, nb);
                if a != b {
                    parent[b] = a;
                    size[a] += size[b];
                }
            }
        }
        let mut seen = vec![false; n];
        for j in 0..n {
            if size[j] > 0 {
                let r = find(&mut parent, j);
                if !seen[r] {
                    seen[r] = true;
                    if size[r] == m as usize {
                        ans = (step + 1) as i32;
                    }
                }
            }
        }
    }
    ans
}

fn main() {
    println!("{}", find_latest_step(vec![3, 5, 1, 2, 4], 1));
}

#[cfg(test)]
mod tests {
    use super::find_latest_step;

    #[test]
    fn example_one() {
        assert_eq!(find_latest_step(vec![3, 5, 1, 2, 4], 1), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_latest_step(vec![3, 1, 5, 4, 2], 2), -1);
    }
}
