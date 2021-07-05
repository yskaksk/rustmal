use crate::reader::read_str;
use crate::types::MalType;
use crate::printer::pr_str;

fn READ(s: &str) -> MalType {
    let r = read_str(s);
    return r
}

fn EVAL(mal: MalType) -> MalType {
    return mal
}

fn PRINT(mal: MalType) -> String {
    return pr_str(&mal)
}

pub fn rep(s: &str) -> String {
    return PRINT(EVAL(READ(s)))
}

