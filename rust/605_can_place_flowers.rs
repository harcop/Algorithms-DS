/// LeetCode #605 - Can Place Flowers
fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut fb = flowerbed;
    let mut need = n;
    let len = fb.len();
    for i in 0..len {
        if need <= 0 {
            break;
        }
        if fb[i] == 1 {
            continue;
        }
        let left_ok = i == 0 || fb[i - 1] == 0;
        let right_ok = i + 1 == len || fb[i + 1] == 0;
        if left_ok && right_ok {
            fb[i] = 1;
            need -= 1;
        }
    }
    need <= 0
}

fn main() {
    println!("{}", can_place_flowers(vec![1, 0, 0, 0, 1], 1));
}

#[cfg(test)]
mod tests {
    use super::can_place_flowers;

    #[test]
    fn example_one() {
        assert!(can_place_flowers(vec![1, 0, 0, 0, 1], 1));
    }

    #[test]
    fn example_two() {
        assert!(!can_place_flowers(vec![1, 0, 0, 0, 1], 2));
    }
}
