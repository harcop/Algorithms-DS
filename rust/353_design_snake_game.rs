/// LeetCode #353 - Design Snake Game
use std::collections::{HashSet, VecDeque};

struct SnakeGame {
    food: Vec<Vec<i32>>,
    fi: usize,
    rows: i32,
    cols: i32,
    snake: VecDeque<(i32, i32)>,
    occupied: HashSet<(i32, i32)>,
}

impl SnakeGame {
    fn new(width: i32, height: i32, food: Vec<Vec<i32>>) -> Self {
        let mut snake = VecDeque::new();
        snake.push_back((0, 0));
        SnakeGame {
            food,
            fi: 0,
            rows: height,
            cols: width,
            occupied: HashSet::from([(0, 0)]),
            snake,
        }
    }

    fn mv(&mut self, direction: String) -> i32 {
        let &(r, c) = self.snake.front().unwrap();
        let (nr, nc) = match direction.as_str() {
            "U" => (r - 1, c),
            "D" => (r + 1, c),
            "L" => (r, c - 1),
            _ => (r, c + 1),
        };
        if nr < 0 || nc < 0 || nr >= self.rows || nc >= self.cols {
            return -1;
        }
        let eating = self.fi < self.food.len()
            && self.food[self.fi][0] == nr
            && self.food[self.fi][1] == nc;

        if !eating {
            let tail = self.snake.back().cloned().unwrap();
            self.snake.pop_back();
            self.occupied.remove(&tail);
        }

        if self.occupied.contains(&(nr, nc)) {
            return -1;
        }
        self.snake.push_front((nr, nc));
        self.occupied.insert((nr, nc));
        if eating {
            self.fi += 1;
        }

        self.snake.len() as i32 - 1
    }
}

fn main() {
    let mut g = SnakeGame::new(3, 2, vec![vec![1, 2], vec![0, 1]]);
    println!("{}", g.mv("R".into()));
}

#[cfg(test)]
mod tests {
    use super::SnakeGame;

    #[test]
    fn example() {
        let mut g = SnakeGame::new(3, 2, vec![vec![1, 2], vec![0, 1]]);
        assert_eq!(g.mv("R".into()), 0);
        assert_eq!(g.mv("D".into()), 0);
        assert_eq!(g.mv("R".into()), 1);
        assert_eq!(g.mv("U".into()), 1);
        assert_eq!(g.mv("L".into()), 2);
        assert_eq!(g.mv("U".into()), -1);
    }
}
