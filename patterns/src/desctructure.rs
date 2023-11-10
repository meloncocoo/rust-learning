pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn x(self) -> i32 {
        self.x
    }

    pub fn y(self) -> i32 {
        self.y
    }
}

pub enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        let p = Point::new(0, 7);

        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);
    }

    #[test]
    fn test_specified_field() {
        let p = Point::new(10, 0);

        match p {
            Point { x, y: 0 } => println!("On the x axis at {x}"),
            Point { x: 0, y } => println!("On the y axis at {y}"),
            Point { x, y } => {
                println!("On neither axis: ({x}, {y})");
            }
        }
    }

    #[test]
    fn test_enum() {
        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {
            Message::Quit => {
                println!("The Quit variant has no to destructure.");
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {x} and in the y direction {y}");
            }
            Message::Write(text) => {
                println!("Text message: {text}");
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change the color to hue {h}, saturation {s}, and value {v}");
            }
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change the color to red {r}, green {g}, and blue {b}");
            }
        }
    }

    #[test]
    fn test_match_guard() {
        let num = Some(5);

        match num {
            Some(x) if x % 2 == 0 => println!("The num {} is even", x),
            Some(x) => println!("The num {} is odd", x),
            None => (),
        }
    }

    #[test]
    fn test_match_bind() {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 12 };

        match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
            } => {
                println!("Found an id in range: {}", id_variable);
            }
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range");
            }
            Message::Hello { id } => {
                println!("Found some other id: {}", id);
            }
        }
    }
}
