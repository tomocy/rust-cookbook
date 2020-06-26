#[derive(Debug, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    #[allow(dead_code)]
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[allow(dead_code)]
    fn proceed(&mut self, cmd: &Command) {
        match cmd {
            Command::Up(y) => self.y += y,
            Command::Left(x) => self.x -= x,
            Command::Right(x) => self.x += x,
            Command::Down(y) => self.y -= y,
        }
    }
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum Command {
    Up(i32),
    Left(i32),
    Right(i32),
    Down(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut pos = Position::new(0, 0);

        [
            Command::Right(3),
            Command::Down(1),
            Command::Up(6),
            Command::Left(5),
        ]
        .iter()
        .for_each(|cmd| pos.proceed(cmd));

        assert_eq!(Position::new(-2, 5), pos);
    }
}
