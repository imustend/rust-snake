use std::fmt::{Display, Write};

use crate::{random::random_range, HashSet};

type Position = (isize, isize);


fn add_positions((x1, y1): Position, (x2, y2): Position) -> Position {
	(x1 + x2, y1 + y2)
}



#[derive(Debug)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug)]
pub struct Snake {
    pub width: isize,
    pub height: isize,
    snake_parts: Vec<Position>,
    snake_head: Position,
    pub snake_direction: Direction,
    fruit: HashSet<Position>,
	pub lose: bool,
	number_of_fruits: usize,
}

impl Display for Snake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);
                if self.snake_head == pos {
                    f.write_str("❎")?;
                } else if self.snake_parts.contains(&pos) {
                    f.write_str("🟩")?;
                } else if self.fruit.contains(&pos) {
                    f.write_str("🍌")?;
                } else {
                    f.write_str("🟪")?;
                }
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Snake {
    pub fn new(width: isize, height: isize, num_of_friuts: usize, starting_size: usize) -> Snake {
        let mut parts = Vec::new();

        while parts.len() < starting_size {
            println!("{}, {}", width / 2 - parts.len() as isize - 1, height / 2);
            // parts.push();
			parts.insert(0, (width / 2 - parts.len() as isize- 1, height / 2));
		}

		println!("{}", parts.len());
        Snake {
            width,
            height,
            snake_parts: parts.clone(),
            snake_head: (width / 2, height / 2),
            snake_direction: Direction::Right,
            fruit: {
                let mut fruit = HashSet::new();

                while fruit.len() < num_of_friuts {
                    let pos = (random_range(0, width as usize), random_range(0, height as usize));
                    if parts.contains(&pos) || (width / 2, height / 2) == pos {
                        continue;
                    }
                    fruit.insert(pos);
                }

                fruit
            },
			lose: false,
			number_of_fruits: num_of_friuts,
        }
    }

	pub fn move_snake(&mut self) {
		let parts_len = self.snake_parts.len();
		let parts_temp = self.snake_parts.clone();
		for (i, c) in self.snake_parts.iter_mut().enumerate() {
			if i == parts_len - 1 {
				*c = self.snake_head;
			} else {
				*c = parts_temp[i + 1];
			}
		}
		let move_head_by: Position = match &self.snake_direction {
			Direction::Up => (0, -1),
			Direction::Left => (-1, 0),
			Direction::Down => (0, 1),
			Direction::Right => (1, 0),
		};
		// self.snake_head = (self.snake_head.0 + , self.snake_head.1);
		self.snake_head = add_positions(self.snake_head, move_head_by);
	}

	// i have no idea if it works but looks about right
	pub fn grow_snake(&mut self) {
		self.snake_parts.insert(0, self.snake_parts[0].clone());
	}

	pub fn check_if_lost(&mut self) {
		let collided_with_self = self.snake_parts.contains(&self.snake_head);
		let collided_with_wall = !(0..self.width).contains(&self.snake_head.0) || !(0..self.height).contains(&self.snake_head.1);

		self.lose = collided_with_self || collided_with_wall;
	}

	pub fn spawn_fruit(&mut self) {
		while self.fruit.len() < self.number_of_fruits {
			let pos = (random_range(0, self.width as usize), random_range(0, self.height as usize));
			if self.snake_parts.contains(&pos) || (self.width / 2, self.height / 2) == pos {
				continue;
			}
			self.fruit.insert(pos);
		}
	}

	pub fn grow_when_ate(&mut self) {
		if self.fruit.contains(&self.snake_head) {
			self.grow_snake();
			self.fruit.remove(&self.snake_head);
			self.spawn_fruit();
		}
	}

	pub fn get_len(&self) -> usize {
		self.snake_parts.len() + 1
	}

}

#[cfg(test)]
mod tests {
    use crate::snake::Direction;

    use super::Snake;

    #[test]
    fn test() {
        let mut snake = Snake::new(15, 15, 2, 3);
		snake.move_snake();
		snake.move_snake();
		snake.grow_snake();
		snake.move_snake();
		snake.snake_direction = Direction::Up;
		snake.move_snake();
		snake.move_snake();

        println!("{}", snake);
		println!("{:?}", snake);
    }
}
