fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Outcome([Match; 4]);

#[derive(Debug)]
enum Match {
    Full,
    Partial,
    Not,
}

#[derive(Debug, PartialEq, Eq)]
enum Peg {
    Black,
    White,
    Red,
    Blue,
    Green,
    Yellow,
}

#[derive(Debug, PartialEq, Eq)]
struct Position([Peg; 4]);

impl Position {
    fn compare(&self, other: &Self) -> Outcome {
        todo!()
    }
}
