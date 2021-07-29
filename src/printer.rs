use crate::types::MalType::{self, *};

pub fn pr_str(mt: &MalType) -> String {
    match mt {
        MalNumber(val) => val.to_string(),
        MalSymbol(val) | MalKeyword(val) | MalError(val) => val.to_string(),
        MalBool(val) => val.to_string(),
        MalNil => String::from("Nil"),
        MalList { elm } => print_listlike(&elm, String::from("("), String::from(")")),
        MalVector { elm } => print_listlike(&elm, String::from("["), String::from("]")),
    }
}

fn print_listlike(elm: &Vec<MalType>, left: String, right: String) -> String {
    let mut s = left;
    let mut i = 0;
    for el in elm {
        s += &pr_str(el);
        if i < elm.len() - 1 {
            s += " ";
        }
        i += 1;
    }
    s += &right;
    return s;
}
