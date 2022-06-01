use std::env;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use svg::{
    node::element::{
        path::{Command, Data, Position},
        Path, Rectangle,
    },
    Document,
};

const HEIGHT: isize = 400;
const WIDTH: isize = 400;
const HOME_X: isize = WIDTH / 2;
const HOME_Y: isize = HEIGHT / 2;
const STROKE_WIDTH: usize = 5;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Forward(isize),
    TurnLeft,
    TurnRight,
    Home,
    Noop(u8),
}

#[derive(Debug, Clone, Copy)]
enum Orientation {
    North,
    East,
    West,
    South,
}
#[derive(Debug)]
struct Artist {
    x: isize,
    y: isize,
    heading: Orientation,
}

impl Artist {
    fn new() -> Artist {
        Artist {
            x: HOME_X,
            y: HOME_Y,
            heading: Orientation::North,
        }
    }

    fn home(&mut self) {
        self.x = HOME_X;
        self.y = HOME_Y;
    }

    fn forward(&mut self, distance: isize) {
        match self.heading {
            Orientation::North => self.y += distance,
            Orientation::East => self.x -= distance,
            Orientation::West => self.x += distance,
            Orientation::South => self.y += distance,
        }
    }
    fn turn_right(&mut self) {
        self.heading = match self.heading {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::West => Orientation::North,
            Orientation::South => Orientation::West,
        }
    }

    fn turn_left(&mut self) {
        self.heading = match self.heading {
            Orientation::North => Orientation::West,
            Orientation::East => Orientation::North,
            Orientation::West => Orientation::South,
            Orientation::South => Orientation::East,
        }
    }

    fn wrap(&mut self) {
        if self.x < 0 {
            self.x = HOME_X;
            self.heading = Orientation::West
        } else if self.x > WIDTH {
            self.x = HOME_X;
            self.heading = Orientation::East
        }

        if self.y < 0 {
            self.y = HOME_Y;
            self.heading = Orientation::North
        } else if self.y > HEIGHT {
            self.y = HOME_Y;
            self.heading = Orientation::South
        }
    }
}

fn parse(input: &str) -> Vec<Operation> {
    input
        .as_bytes()
        .par_iter()
        .map(|byte| {
            match byte {
                b'0' => Operation::Home,
                b'1'..=b'9' => {
                    let distance = (byte - 0x30) as isize; // ASCI starts at 0x30(48)
                    Operation::Forward(distance * (HEIGHT / 10))
                }
                b'a' | b'b' | b'c' => Operation::TurnLeft,
                b'd' | b'e' | b'f' => Operation::TurnRight,
                _ => Operation::Noop(*byte),
            }
        })
        .collect()
}

fn convert(ops: &Vec<Operation>) -> Vec<Command> {
    let mut turtle = Artist::new();
    let mut path_data = Vec::<Command>::with_capacity(ops.len());

    let start_at_home = Command::Move(Position::Absolute, (HOME_X, HOME_Y).into());
    path_data.push(start_at_home);

    for op in ops {
        match *op {
            Operation::Forward(distance) => turtle.forward(distance),
            Operation::TurnLeft => turtle.turn_left(),
            Operation::TurnRight => turtle.turn_right(),
            Operation::Home => turtle.home(),
            Operation::Noop(byte) => eprintln!("WARNING: {:?} is not allowed", byte),
        };

        let path_segment = Command::Line(Position::Absolute, (turtle.x, turtle.y).into());
        path_data.push(path_segment);
        turtle.wrap();
    }
    path_data
}

fn gen_svg(path_data: Vec<Command>) -> Document {
    let background = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", WIDTH)
        .set("height", HEIGHT)
        .set("fill", "#ffffff");

    let border = background
        .clone()
        .set("fill-opacity", "0.0")
        .set("stroke", "#cccccc")
        .set("stroke-width", 3 * STROKE_WIDTH);

    let sketch = Path::new()
        .set("fill", "none")
        .set("stroke", "#2f2f2f")
        .set("stroke-width", STROKE_WIDTH)
        .set("stroke-opacity", "0.9")
        .set("d", Data::from(path_data));

    let document = Document::new()
        .set("viewBox", (0, 0, HEIGHT, WIDTH))
        .set("height", HEIGHT)
        .set("width", WIDTH)
        .set("style", "style=\"outline: 5px solid #800000;\"")
        .add(background)
        .add(sketch)
        .add(border);
    document
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = args.get(1).expect("Please provide hex input");
    let default_filename = format!("{}.svg", input);
    let save_to = args.get(2).unwrap_or(&default_filename);

    let ops = parse(&input);
    let path_data = convert(&ops);
    let doc = gen_svg(path_data);
    svg::save(save_to, &doc).expect("Wasn't able to save svg");
}
