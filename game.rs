use crossterm::{
    cursor::{Hide, MoveTo},
    event::{poll, read, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    Result,
};
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

const GAME_WIDTH: u16 = 40;
const GAME_HEIGHT: u16 = 20;
const SNAKE_COLOR: Color = Color::Cyan;
const FOOD_COLOR: Color = Color::Yellow;

struct Game {
    snake: Snake,
    food: Food,
    direction: Direction,
    game_over: bool,
}

struct Snake {
    body: Vec<(u16, u16)>,
}

struct Food {
    x: u16,
    y: u16,
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Game {
    fn new() -> Self {
        let snake = Snake {
            body: vec![(2, 2), (3, 2), (4, 2)],
        };
        let food = Food { x: 10, y: 10 };
        let direction = Direction::Right;
        let game_over = false;

        Game {
            snake,
            food,
            direction,
            game_over,
        }
    }

    fn update(&mut self) {
        let (head_x, head_y) = self.snake.body[0];
        let new_head = match self.direction {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        };

        if new_head.0 >= GAME_WIDTH || new_head.1 >= GAME_HEIGHT {
            self.game_over = true;
            return;
        }

         if self.snake.body.iter().any(|&(x, y)| x == new_head.0 && y == new_head.1) {
            self.game_over = true;
            return;
        }

        self.snake.body.insert(0, new_head);

        if self.snake.body[0].0 == self.food.x && self.snake.body[0].1 == self.food.y {
            self.food.x = rand::random::<u16>() % GAME_WIDTH;
            self.food.y = rand::random::<u16>() % GAME_HEIGHT;
        } else {
            self.snake.body.pop();
        }
    }
}

fn draw_game(game: &Game) -> Result<()> {
    let mut stdout = stdout();

    execute!(stdout, Clear(ClearType::All), Hide)?;
    execute!(stdout, MoveTo(0, 0))?;

    for y in 0..=GAME_HEIGHT {
        for x in 0..=GAME_WIDTH {
            if x == 0 || x == GAME_WIDTH || y == 0 || y == GAME_HEIGHT {
                execute!(
                    stdout,
                    SetForegroundColor(Color::White),
                    SetBackgroundColor(Color::DarkGrey),
                    Print("#")
                )?;
            } else if game.snake.body.iter().any(|&(sx, sy)| sx == x && sy == y) {
                execute!(
                    stdout,
                    SetForegroundColor(Color::White),
                    SetBackgroundColor(SNAKE_COLOR),
                    Print("O")
                )?;
            } else if game.food.x == x && game.food.y == y {
                execute!(
                    stdout,
                    SetForegroundColor(Color::Black),
                    SetBackgroundColor(FOOD_COLOR),
                    Print("@")
                )?;
            } else {
                execute!(stdout, ResetColor, Print(" "))?;
            }
        }
        execute!(stdout, Print("\r\n"))?;
    }

    stdout.flush()?;
    Ok(())
}

fn process_input(game: &mut Game) -> Result<()> {
    if poll(Duration::from_millis(100))? {
        if let Event::Key(key_event) = read()? {
            match key_event.code {
                KeyCode::Char('q') => game.game_over = true,
                KeyCode::Up => {
                    if game.direction != Direction::Down {
                        game.direction = Direction::Up;
                    }
                }
                KeyCode::Down => {
                    if game.direction != Direction::Up {
                        game.direction = Direction::Down;
                    }
                }
                KeyCode::Left => {
                    if game.direction != Direction::Right {
                        game.direction = Direction::Left;
                    }
                }
                KeyCode::Right => {
                    if game.direction != Direction::Left {
                        game.direction = Direction::Right;
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();

    let mut game = Game::new();

    loop {
        if game.game_over {
            break;
        }

        let start_time = Instant::now();

        process_input(&mut game)?;
        game.update();
        draw_game(&game)?;

        let elapsed_time = Instant::now().duration_since(start_time);
        if elapsed_time < Duration::from_millis(100) {
            std::thread::sleep(Duration::from_millis(100) - elapsed_time);
        }
    }

    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
    disable_raw_mode()?;
    Ok(())
}
