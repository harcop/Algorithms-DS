/// LeetCode #2069 - Walking Robot Simulation II
pub struct Robot {
    mx: i32,
    my: i32,
    p: i32,
    cur: i32,
    moved: bool,
}

impl Robot {
    fn new(width: i32, height: i32) -> Self {
        let mx = width - 1;
        let my = height - 1;
        Robot {
            mx,
            my,
            p: 2 * mx + 2 * my,
            cur: 0,
            moved: false,
        }
    }

    fn step(&mut self, num: i32) {
        self.moved = true;
        if self.p > 0 {
            self.cur = (self.cur + num) % self.p;
        }
    }

    fn get_pos(&self) -> Vec<i32> {
        let d = self.cur;
        let mx = self.mx;
        let my = self.my;
        if d <= mx {
            return vec![d, 0];
        }
        if d <= mx + my {
            return vec![mx, d - mx];
        }
        if d <= 2 * mx + my {
            return vec![mx - (d - (mx + my)), my];
        }
        vec![0, my - (d - (2 * mx + my))]
    }

    fn get_dir(&self) -> String {
        if !self.moved {
            return "East".into();
        }
        let d = self.cur;
        let mx = self.mx;
        let my = self.my;
        if (1..=mx).contains(&d) {
            "East".into()
        } else if d <= mx + my {
            "North".into()
        } else if d <= 2 * mx + my {
            "West".into()
        } else {
            "South".into()
        }
    }
}

fn main() {
    let mut robot = Robot::new(6, 3);
    robot.step(2);
    robot.step(2);
    println!("{:?} {}", robot.get_pos(), robot.get_dir());
}

#[cfg(test)]
mod tests {
    use super::Robot;

    #[test]
    fn example_sequence() {
        let mut robot = Robot::new(6, 3);
        robot.step(2);
        robot.step(2);
        assert_eq!(robot.get_pos(), vec![4, 0]);
        assert_eq!(robot.get_dir(), "East");
        robot.step(2);
        robot.step(1);
        robot.step(4);
        assert_eq!(robot.get_pos(), vec![1, 2]);
        assert_eq!(robot.get_dir(), "West");
    }

    #[test]
    fn starts_east() {
        let robot = Robot::new(3, 3);
        assert_eq!(robot.get_pos(), vec![0, 0]);
        assert_eq!(robot.get_dir(), "East");
    }
}
