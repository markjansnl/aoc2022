pub mod input;

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for RockPaperScissors {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Wrong input!"),
        }
    }
}
