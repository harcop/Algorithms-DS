/// LeetCode #502 - IPO
use std::collections::BinaryHeap;

fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    let mut projects: Vec<(i32, i32)> = capital.into_iter().zip(profits).collect();
    projects.sort_unstable_by_key(|&(c, _)| c);
    let mut heap = BinaryHeap::new();
    let mut i = 0;
    let mut w = w;
    for _ in 0..k {
        while i < projects.len() && projects[i].0 <= w {
            heap.push(projects[i].1);
            i += 1;
        }
        if let Some(p) = heap.pop() {
            w += p;
        } else {
            break;
        }
    }
    w
}

fn main() {
    println!(
        "{}",
        find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1])
    );
}

#[cfg(test)]
mod tests {
    use super::find_maximized_capital;

    #[test]
    fn example_one() {
        assert_eq!(
            find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2]),
            6
        );
    }
}
