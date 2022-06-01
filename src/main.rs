use console::Term;
use std::{
	collections::HashSet,
	sync::{
		mpsc::{
			self,
			Receiver,
			TryRecvError
		}
	},
	thread,
	time::Duration,
};

mod random;
mod snake;

use crate::snake::{Direction, Snake};

fn spawn_stdin_channel() -> Receiver<char> {
    let (tx, rx) = mpsc::channel::<char>();
    thread::spawn(move || loop {
        let term = Term::stdout();
        let mut temp = '_';
        if let Ok(character) = term.read_char() {
            temp = character;
        }

        tx.send(temp).unwrap();
    });
    rx
}

fn main() {
    let mut snake = Snake::new(10, 10, 2, 3);
    let term = Term::stdout();
    let _res = term.hide_cursor();

	// yes i used zero width space, it works
    let mut s_key = '​';

    let stdin_channel = spawn_stdin_channel();

    while !snake.lose {
        let _res = term.clear_screen();
        println!("{}", snake);
        snake.grow_when_ate();
        println!("Points: {}", snake.get_len());

        // check for key press
        // some multi threading shenanigans 
        match stdin_channel.try_recv() {
            Ok(key) => {
                s_key = key;
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }

        match s_key {
			'w' => snake.snake_direction = Direction::Up,
            'a' => snake.snake_direction = Direction::Left,
            's' => snake.snake_direction = Direction::Down,
            'd' => snake.snake_direction = Direction::Right,
            _ => {}
        }
		// here again, the zero width space
        if s_key == '​' {
			println!("Press any key");
			thread::sleep(Duration::from_millis(500));
            continue;
        }
		
		snake.move_snake();

        snake.check_if_lost();
        thread::sleep(Duration::from_millis(500));
    }
    println!("you lose");
}
