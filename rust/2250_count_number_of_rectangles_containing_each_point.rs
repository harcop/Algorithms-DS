/// LeetCode #2250 - Count Number of Rectangles Containing Each Point
fn count_rectangles(rectangles: Vec<Vec<i32>>, points: Vec<Vec<i32>>) -> Vec<i32> {
    let mut y_to_xs: Vec<Vec<i32>> = vec![Vec::new(); 101];

    for rect in rectangles {
        y_to_xs[rect[1] as usize].push(rect[0]);
    }

    for xs in &mut y_to_xs {
        xs.sort_unstable();
    }

    points
        .iter()
        .map(|point| {
            let x = point[0];
            let y = point[1] as usize;
            let mut count = 0;
            for height in y..=100 {
                let xs = &y_to_xs[height];
                count += (xs.len() - xs.partition_point(|&length| length < x)) as i32;
            }
            count
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        count_rectangles(
            vec![vec![1, 2], vec![2, 3], vec![2, 5]],
            vec![vec![2, 1], vec![1, 4]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::count_rectangles;

    #[test]
    fn example_one() {
        assert_eq!(
            count_rectangles(
                vec![vec![1, 2], vec![2, 3], vec![2, 5]],
                vec![vec![2, 1], vec![1, 4]]
            ),
            vec![2, 1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            count_rectangles(vec![vec![1, 1], vec![2, 2], vec![3, 3]], vec![vec![1, 3], vec![1, 1]]),
            vec![1, 3]
        );
    }
}
