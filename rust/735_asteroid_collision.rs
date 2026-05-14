/// LeetCode #735 - Asteroid Collision
fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut st: Vec<i32> = vec![];
    for a in asteroids {
        if a > 0 {
            st.push(a);
            continue;
        }
        loop {
            let last = st.last().copied();
            match last {
                Some(x) if x < 0 => {
                    st.push(a);
                    break;
                }
                Some(x) if x > -a => break,
                Some(x) if x == -a => {
                    st.pop();
                    break;
                }
                Some(_) => {
                    st.pop();
                }
                None => {
                    st.push(a);
                    break;
                }
            }
        }
    }
    st
}

fn main() {
    println!("{:?}", asteroid_collision(vec![5, 10, -5]));
}

#[cfg(test)]
mod tests {
    use super::asteroid_collision;

    #[test]
    fn example_one() {
        assert_eq!(asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
    }

    #[test]
    fn example_two() {
        assert_eq!(asteroid_collision(vec![8, -8]), vec![]);
    }
}
