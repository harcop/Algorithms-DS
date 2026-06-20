/// LeetCode #1996 - The Number of Weak Characters in the Game
fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
    let mut properties = properties;
    properties.sort_by(|a, b| {
        if a[0] != b[0] {
            b[0].cmp(&a[0])
        } else {
            a[1].cmp(&b[1])
        }
    });
    let mut ans = 0i32;
    let mut mx = 0i32;
    for p in properties {
        if p[1] < mx {
            ans += 1;
        }
        mx = mx.max(p[1]);
    }
    ans
}

fn main() {
    println!(
        "{}",
        number_of_weak_characters(vec![vec![5, 5], vec![6, 3], vec![3, 6]])
    );
}

#[cfg(test)]
mod tests {
    use super::number_of_weak_characters;

    #[test]
    fn example_one() {
        assert_eq!(
            number_of_weak_characters(vec![vec![5, 5], vec![6, 3], vec![3, 6]]),
            0
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_weak_characters(vec![vec![2, 2], vec![3, 3]]), 1);
    }
}
