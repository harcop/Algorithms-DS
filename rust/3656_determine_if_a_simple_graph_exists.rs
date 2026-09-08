/// LeetCode #3656 - Determine if a Simple Graph Exists (premium)
fn simple_graph_exists(degrees: Vec<i32>) -> bool {
    let n = degrees.len();
    let mut d = degrees;
    d.sort_unstable_by(|a, b| b.cmp(a));
    let sum: i64 = d.iter().map(|&x| x as i64).sum();
    if sum % 2 != 0 {
        return false;
    }
    let mut pref = vec![0i64; n + 1];
    for i in 0..n {
        pref[i + 1] = pref[i] + d[i] as i64;
    }
    let mut j = n as i32 - 1;
    for k in 1..=n {
        while j >= k as i32 && d[j as usize] < k as i32 {
            j -= 1;
        }
        let right = if j >= k as i32 {
            (k as i64) * (j as i64 - k as i64 + 1) + (pref[n] - pref[j as usize + 1])
        } else {
            pref[n] - pref[k]
        };
        if pref[k] > (k as i64) * (k as i64 - 1) + right {
            return false;
        }
    }
    true
}

fn main() {
    println!("{}", simple_graph_exists(vec![3, 1, 2, 2]));
}

#[cfg(test)]
mod tests {
    use super::simple_graph_exists;

    #[test]
    fn example1() {
        assert!(simple_graph_exists(vec![3, 1, 2, 2]));
    }

    #[test]
    fn example2() {
        assert!(!simple_graph_exists(vec![1, 3, 3, 1]));
    }
}
