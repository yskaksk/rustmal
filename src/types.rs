#[derive(Clone, PartialEq, Debug)]
pub enum MalType {
    MalList { elm: Vec<MalType> },
    MalNumber(i32),
    MalSymbol(String),
}
