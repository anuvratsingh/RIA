#[derive(Debug, Clone, Copy)]
enum Operation {
    Forward(isize),
    TurnLeft,
    TurnRight,
    Home,
    Noop(usize),
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

fn parse(input: &str) -> Vec<Operation> {
    let mut steps = Vec::<Operation>::new();
    for byte in input.bytes() {
        let step = match byte {
            b'0' => Operation::Home,
            b'1'..=b'9' => {
                let distance = (byte - 0x30) as isize;
                Operation::Forward(distance * (HEIGHT / 10))
            }
        };
    }
    steps
}

fn main() {
    println!("Hello, world!");
}
