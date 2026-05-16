/// LeetCode #832 - Flipping an Image
fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    image
        .into_iter()
        .map(|row| row.into_iter().map(|x| 1 - x).rev().collect())
        .collect()
}

fn main() {
    println!("{:?}", flip_and_invert_image(vec![vec![1, 1, 0], vec![1, 0, 1]]));
}

#[cfg(test)]
mod tests {
    use super::flip_and_invert_image;

    #[test]
    fn example_one() {
        assert_eq!(
            flip_and_invert_image(vec![vec![1, 1, 0], vec![1, 0, 1]]),
            vec![vec![1, 0, 0], vec![0, 1, 0]]
        );
    }
}
