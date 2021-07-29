#[derive(Clone, PartialEq, Debug)]
pub enum MalType {
    MalList { elm: Vec<MalType> },
    MalVector { elm: Vec<MalType> },
    MalNumber(i32),
    MalSymbol(String),
    MalBool(bool),
    MalNil,
    MalError(String),
    MalKeyword(String),
}
