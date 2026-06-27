/// LeetCode #2120 - Execution of All Suffix Instructions Staying in a Grid
fn execute_instructions(n: i32, start_pos: Vec<i32>, s: String) -> Vec<i32> {
    let instructions = s.as_bytes();
    let mut ans = Vec::with_capacity(instructions.len());

    for i in 0..instructions.len() {
        let mut row = start_pos[0];
        let mut col = start_pos[1];
        let mut steps = 0;

        for &instruction in &instructions[i..] {
            match instruction {
                b'L' => col -= 1,
                b'R' => col += 1,
                b'U' => row -= 1,
                b'D' => row += 1,
                _ => unreachable!(),
            }

            if row < 0 || row >= n || col < 0 || col >= n {
                break;
            }
            steps += 1;
        }

        ans.push(steps);
    }

    ans
}

fn main() {
    println!("{:?}", execute_instructions(3, vec![0, 1], "RRDDLU".into()));
}

#[cfg(test)]
mod tests {
    use super::execute_instructions;

    #[test]
    fn example_one() {
        assert_eq!(
            execute_instructions(3, vec![0, 1], "RRDDLU".into()),
            vec![1, 5, 4, 3, 1, 0]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            execute_instructions(2, vec![1, 1], "LURD".into()),
            vec![4, 1, 0, 0]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            execute_instructions(1, vec![0, 0], "LRUD".into()),
            vec![0, 0, 0, 0]
        );
    }
}
