#[derive(Debug, Default)]
struct Outcome([Option<Hit>; 4]);

#[derive(Debug, PartialEq, Eq)]
enum Hit {
    Full,
    Partial,
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
        let mut outcome = Outcome::default();
        for (i, peg) in self.0.iter().enumerate() {
            if peg == &other.0[i] {
                outcome.0[i] = Some(Hit::Full)
            }
        }
        for (i, peg) in self.0.iter().enumerate() {
            for (j, other_peg) in other.0.iter().enumerate() {
                if i != j {
                    todo!()
                }
            }
        }
        todo!()
    }
}
