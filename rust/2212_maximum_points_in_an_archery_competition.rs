/// LeetCode #2212 - Maximum Points in an Archery Competition
fn maximum_bob_points(num_arrows: i32, alice_arrows: Vec<i32>) -> Vec<i32> {
    const FULL_MASK: i32 = (1 << 12) - 1;
    let mut max_point = 0i32;
    let mut max_mask = 0i32;

    for mask in 0..FULL_MASK {
        let (shotable, point) = shotable_and_point(mask, num_arrows, &alice_arrows);
        if shotable && point > max_point {
            max_point = point;
            max_mask = mask;
        }
    }

    bobs_arrows(max_mask, num_arrows, &alice_arrows)
}

fn shotable_and_point(mask: i32, mut left_arrows: i32, alice_arrows: &[i32]) -> (bool, i32) {
    let mut point = 0i32;
    for i in 0..12 {
        if mask >> i & 1 == 1 {
            left_arrows -= alice_arrows[i] + 1;
            point += i as i32;
        }
    }
    (left_arrows >= 0, point)
}

fn bobs_arrows(mask: i32, mut left_arrows: i32, alice_arrows: &[i32]) -> Vec<i32> {
    let mut bobs = vec![0i32; 12];
    for i in 0..12 {
        if mask >> i & 1 == 1 {
            bobs[i] = alice_arrows[i] + 1;
            left_arrows -= alice_arrows[i] + 1;
        }
    }
    bobs[0] = left_arrows;
    bobs
}

fn main() {
    println!(
        "{:?}",
        maximum_bob_points(9, vec![1, 1, 0, 1, 0, 0, 2, 1, 0, 1, 2, 0])
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_bob_points;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_bob_points(9, vec![1, 1, 0, 1, 0, 0, 2, 1, 0, 1, 2, 0]),
            vec![0, 0, 0, 0, 1, 1, 0, 0, 1, 2, 3, 1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_bob_points(3, vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2]), vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0]);
    }
}
