/// LeetCode #835 - Image Overlap
use std::collections::HashMap;

fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
    let n = img1.len();
    let mut a = Vec::new();
    let mut b = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if img1[i][j] == 1 {
                a.push((i as i32, j as i32));
            }
            if img2[i][j] == 1 {
                b.push((i as i32, j as i32));
            }
        }
    }
    let mut count = HashMap::new();
    let mut best = 0;
    for &(r1, c1) in &a {
        for &(r2, c2) in &b {
            let d = (r1 - r2, c1 - c2);
            let e = count.entry(d).or_insert(0);
            *e += 1;
            best = best.max(*e);
        }
    }
    best
}

fn main() {
    let img1 = vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]];
    let img2 = vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]];
    println!("{}", largest_overlap(img1, img2));
}

#[cfg(test)]
mod tests {
    use super::largest_overlap;

    #[test]
    fn example_one() {
        let img1 = vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]];
        let img2 = vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]];
        assert_eq!(largest_overlap(img1, img2), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(largest_overlap(vec![vec![1]], vec![vec![1]]), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(largest_overlap(vec![vec![0]], vec![vec![0]]), 0);
    }
}
