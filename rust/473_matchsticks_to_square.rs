/// LeetCode #473 - Matchsticks to Square
fn makesquare(matchsticks: Vec<i32>) -> bool {
    let sum: i32 = matchsticks.iter().sum();
    if sum % 4 != 0 {
        return false;
    }
    let side = sum / 4;
    let mut ms = matchsticks;
    ms.sort_unstable_by(|a, b| b.cmp(a));
    let mut sides = [0i32; 4];

    fn dfs(i: usize, ms: &[i32], target: i32, sides: &mut [i32; 4]) -> bool {
        if i == ms.len() {
            return sides.iter().all(|&s| s == target);
        }
        for k in 0..4 {
            if sides[k] + ms[i] > target {
                continue;
            }
            sides[k] += ms[i];
            if dfs(i + 1, ms, target, sides) {
                return true;
            }
            sides[k] -= ms[i];
            if sides[k] == 0 {
                break;
            }
        }
        false
    }

    dfs(0, &ms, side, &mut sides)
}

fn main() {
    println!("{}", makesquare(vec![1, 1, 2, 2, 2]));
}

#[cfg(test)]
mod tests {
    use super::makesquare;

    #[test]
    fn example_one() {
        assert!(makesquare(vec![1, 1, 2, 2, 2]));
    }

    #[test]
    fn example_two() {
        assert!(!makesquare(vec![3, 3, 3, 3, 4]));
    }
}
