/// LeetCode #370 - Range Addition
fn get_modified_array(length: i32, updates: Vec<Vec<i32>>) -> Vec<i32> {
    let n = length as usize;
    let mut d = vec![0i64; n + 1];
    for u in updates {
        let s = u[0] as usize;
        let e = u[1] as usize;
        let inc = u[2] as i64;
        d[s] += inc;
        d[e + 1] -= inc;
    }
    let mut acc = 0i64;
    (0..n)
        .map(|i| {
            acc += d[i];
            acc as i32
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        get_modified_array(5, vec![vec![1, 3, 2], vec![2, 4, 3], vec![0, 2, -2]])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex() {
        assert_eq!(
            get_modified_array(5, vec![vec![1, 3, 2], vec![2, 4, 3], vec![0, 2, -2]]),
            vec![-2, 0, 3, 5, 3]
        );
    }
}
