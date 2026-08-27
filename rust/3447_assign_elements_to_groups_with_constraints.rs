/// LeetCode #3447 - Assign Elements to Groups with Constraints
fn assign_elements(groups: Vec<i32>, elements: Vec<i32>) -> Vec<i32> {
    let mx = *groups.iter().max().unwrap() as usize;
    let mut d = vec![-1i32; mx + 1];
    for (j, &x) in elements.iter().enumerate() {
        let x = x as usize;
        if x > mx || d[x] != -1 {
            continue;
        }
        let mut y = x;
        while y <= mx {
            if d[y] == -1 {
                d[y] = j as i32;
            }
            y += x;
        }
    }
    groups.into_iter().map(|x| d[x as usize]).collect()
}

fn main() {
    println!("{:?}", assign_elements(vec![8, 4, 3, 2, 4], vec![4, 2]));
}

#[cfg(test)]
mod tests {
    use super::assign_elements;

    #[test]
    fn example1() {
        assert_eq!(
            assign_elements(vec![8, 4, 3, 2, 4], vec![4, 2]),
            vec![0, 0, -1, 1, 0]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            assign_elements(vec![2, 3, 5, 7], vec![5, 3, 3]),
            vec![-1, 1, 0, -1]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            assign_elements(vec![10, 21, 30, 41], vec![2, 1]),
            vec![0, 1, 0, 1]
        );
    }
}
