/// LeetCode #1989 - Maximum Number of People That Can Be Caught in Tag
fn catch_maximum_amountof_people(team: Vec<i32>, dist: i32) -> i32 {
    let mut ans = 0i32;
    let mut j = 0usize;
    let n = team.len();
    for (i, &x) in team.iter().enumerate() {
        if x == 1 {
            while j < n && (team[j] == 1 || i as i32 - j as i32 > dist) {
                j += 1;
            }
            if j < n && (i as i32 - j as i32).abs() <= dist {
                ans += 1;
                j += 1;
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        catch_maximum_amountof_people(vec![0, 1, 0, 1, 0], 3)
    );
}

#[cfg(test)]
mod tests {
    use super::catch_maximum_amountof_people;

    #[test]
    fn example_one() {
        assert_eq!(
            catch_maximum_amountof_people(vec![0, 1, 0, 1, 0], 3),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(catch_maximum_amountof_people(vec![1], 1), 0);
    }
}
