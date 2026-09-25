/// LeetCode #3923 - Minimum Generations to Target Point
fn minimum_generations(points: Vec<Vec<i32>>, target: Vec<i32>) -> i32 {
    let mut gen = [[[-1i32; 7]; 7]; 7];
    let mut list = Vec::new();
    for p in points {
        let (x, y, z) = (p[0] as usize, p[1] as usize, p[2] as usize);
        if gen[x][y][z] == -1 {
            gen[x][y][z] = 0;
            list.push((x, y, z));
        }
    }
    let t = (target[0] as usize, target[1] as usize, target[2] as usize);
    if gen[t.0][t.1][t.2] == 0 {
        return 0;
    }
    let mut i = 0;
    while i < list.len() {
        let (x1, y1, z1) = list[i];
        let g1 = gen[x1][y1][z1];
        for j in 0..i {
            let (x2, y2, z2) = list[j];
            let g2 = gen[x2][y2][z2];
            let c = ((x1 + x2) / 2, (y1 + y2) / 2, (z1 + z2) / 2);
            if gen[c.0][c.1][c.2] == -1 {
                let g = g1.max(g2) + 1;
                gen[c.0][c.1][c.2] = g;
                list.push(c);
                if c == t {
                    return g;
                }
            }
        }
        i += 1;
    }
    -1
}

fn main() {
    println!(
        "{}",
        minimum_generations(vec![vec![0, 0, 0], vec![6, 6, 6]], vec![3, 3, 3])
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_generations;

    #[test]
    fn example1() {
        assert_eq!(
            minimum_generations(vec![vec![0, 0, 0], vec![6, 6, 6]], vec![3, 3, 3]),
            1
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            minimum_generations(vec![vec![0, 0, 0], vec![5, 5, 5]], vec![1, 1, 1]),
            2
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            minimum_generations(
                vec![vec![0, 0, 0], vec![2, 2, 2], vec![3, 3, 3]],
                vec![2, 2, 2]
            ),
            0
        );
    }

    #[test]
    fn example4() {
        assert_eq!(minimum_generations(vec![vec![1, 2, 3]], vec![5, 5, 5]), -1);
    }
}
