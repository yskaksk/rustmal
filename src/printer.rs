use crate::types::MalType;
use crate::types::MalType::{MalList, MalNumber, MalSymbol};


pub fn pr_str(mt: &MalType) -> String {
    match mt {
        MalNumber(val) => format!("{}", val),
        MalSymbol(val) => format!("{}", val),
        MalList {elm} => {
            let mut s = String::from("(");
            let mut i = 0;
            for el in elm {
                s += &pr_str(el);
                if i < elm.len() - 1 {
                    s += " ";
                }
                i += 1;
            }
            s += ")";
            s
        }
    }
}
