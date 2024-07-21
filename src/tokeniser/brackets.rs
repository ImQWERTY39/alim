#[derive(Debug, PartialEq, Eq)]
pub enum Bracket {
    RoundOpen,
    RoundClose,
    SquareOpen,
    SquareClose,
    CurlyOpen,
    CurlyClose,
}
