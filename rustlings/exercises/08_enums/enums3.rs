struct Point {
    x: u64,
    y: u64,
}

// Message describes "what happened" (an event), not the program's state itself.
enum Message {
    Resize { width: u64, height: u64 }, // struct-style variant: named fields
    Move(Point),                        // tuple variant holding another struct
    Echo(String),                       // tuple variant holding a single value
    ChangeColor(u8, u8, u8),            // tuple variant holding 3 positional values
    Quit,                               // unit variant: no data at all
}

struct State {
    width: u64,
    height: u64,
    position: Point,
    message: String,
    // RGB color composed of red, green and blue.
    color: (u8, u8, u8),
    quit: bool,
}

impl State {
    fn resize(&mut self, width: u64, height: u64) {
        self.width = width;
        self.height = height;
    }

    fn move_position(&mut self, point: Point) {
        self.position = point;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn change_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color = (red, green, blue);
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    // process is the dispatcher: it inspects which Message variant arrived
    // and calls the matching state-mutating method above. It returns nothing
    // (`()`) — the point is the side effect on `self`, not a return value.
    fn process(&mut self, message: Message) {
        use Message::*; // lets us write `Resize` instead of `Message::Resize`

        // match must be exhaustive: every variant needs an arm (or `_`).
        // Each arm destructures the variant's data into named bindings,
        // then passes those exact names into the corresponding method call.
        match message {
            Resize { width, height } => self.resize(width, height),
            Move(point) => self.move_position(point),
            Echo(s) => self.echo(s),
            ChangeColor(red, green, blue) => self.change_color(red, green, blue),
            Quit => self.quit(),
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            width: 0,
            height: 0,
            position: Point { x: 0, y: 0 },
            message: String::from("hello world"),
            color: (0, 0, 0),
            quit: false,
        };

        state.process(Message::Resize {
            width: 10,
            height: 30,
        });
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Quit);

        assert_eq!(state.width, 10);
        assert_eq!(state.height, 30);
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.message, "Hello world!");
        assert_eq!(state.color, (255, 0, 255));
        assert!(state.quit);
    }
}
