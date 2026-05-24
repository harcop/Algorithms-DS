/// LeetCode #1298 - Maximum Candies You Can Get from Boxes
use std::collections::VecDeque;

fn max_candies(
    status: Vec<i32>,
    candies: Vec<i32>,
    initial_boxes: Vec<i32>,
    keys: Vec<Vec<i32>>,
    contained_boxes: Vec<Vec<i32>>,
) -> i32 {
    let n = status.len();
    let mut have_box = vec![false; n];
    let mut have_key = vec![false; n];
    let mut opened = vec![false; n];
    for &b in &initial_boxes {
        have_box[b as usize] = true;
    }
    let mut q = std::collections::VecDeque::new();
    let mut try_open = |q: &mut std::collections::VecDeque<usize>, opened: &mut [bool], have_box: &mut [bool], have_key: &mut [bool], status: &[i32]| {
        for i in 0..n {
            if have_box[i] && !opened[i] && (status[i] == 1 || have_key[i]) {
                opened[i] = true;
                have_box[i] = false;
                q.push_back(i);
            }
        }
    };
    try_open(&mut q, &mut opened, &mut have_box, &mut have_key, &status);
    let mut ans = 0;
    while let Some(i) = q.pop_front() {
        ans += candies[i];
        for &k in &keys[i] {
            have_key[k as usize] = true;
        }
        for &b in &contained_boxes[i] {
            have_box[b as usize] = true;
        }
        try_open(&mut q, &mut opened, &mut have_box, &mut have_key, &status);
    }
    ans
}

fn main() {
    println!(
        "{}",
        max_candies(
            vec![1, 0, 1, 0],
            vec![7, 5, 4, 100],
            vec![0],
            vec![vec![], vec![0], vec![1], vec![2]],
            vec![vec![1, 2], vec![3], vec![], vec![]],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::max_candies;

    #[test]
    fn example_one() {
        assert_eq!(
            max_candies(
                vec![1, 0, 1, 0],
                vec![7, 5, 4, 100],
                vec![0],
                vec![vec![], vec![0], vec![1], vec![2]],
                vec![vec![1, 2], vec![3], vec![], vec![]],
            ),
            16
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            max_candies(
                vec![1, 1, 1, 0, 0, 0],
                vec![1, 1, 1, 1, 1, 1],
                vec![0, 1, 2],
                vec![vec![], vec![], vec![], vec![], vec![], vec![]],
                vec![vec![1, 2], vec![2], vec![3, 4], vec![], vec![], vec![]],
            ),
            3
        );
    }
}
