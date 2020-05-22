use std::fmt;

fn main() {
    let p = Point { x: 10, y: 20 };
    OutlinePrinter::print(&p);
}

trait OutlinePrinter: fmt::Display {
    fn print(&self) {
        let s = self.to_string();
        println!("{}", "*".repeat(s.len() + 4));
        println!("*{}*", " ".repeat(s.len() + 2));
        println!("* {} *", s);
        println!("*{}*", " ".repeat(s.len() + 2));
        println!("{}", "*".repeat(s.len() + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrinter for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
