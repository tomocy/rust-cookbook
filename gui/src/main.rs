fn main() {
    let screen = Screen::new(vec![
        Box::new(Title::new(String::from("Hello"))),
        Box::new(SizedBox::new()),
        Box::new(Body::new(String::from("I am writing some Rust code."))),
        Box::new(SizedBox::new()),
        Box::new(SizedBox::new()),
        Box::new(Button::new(String::from("Read more"))),
    ]);
    screen.run();
}

struct Screen {
    components: Vec<Box<dyn Component>>,
}

impl Screen {
    fn new(components: Vec<Box<dyn Component>>) -> Screen {
        Screen { components }
    }

    fn run(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}

trait Component {
    fn draw(&self);
}

struct Title {
    text: String,
}

impl Title {
    fn new(text: String) -> Title {
        Title { text }
    }
}

impl Component for Title {
    fn draw(&self) {
        println!("# {}", self.text);
    }
}

struct Body {
    text: String,
}

impl Body {
    fn new(text: String) -> Body {
        Body { text }
    }
}

impl Component for Body {
    fn draw(&self) {
        println!("{}", self.text);
    }
}

struct Button {
    text: String,
}

impl Button {
    fn new(text: String) -> Button {
        Button { text }
    }
}

impl Component for Button {
    fn draw(&self) {
        println!("| {} |", self.text);
    }
}

struct SizedBox {}

impl SizedBox {
    fn new() -> SizedBox {
        SizedBox {}
    }
}

impl Component for SizedBox {
    fn draw(&self) {
        println!();
    }
}
