use once_cell::sync::OnceCell;
use regex::Regex;

use crate::types::MalType::{self, *};

pub struct Reader {
    tokens: Vec<String>,
    position: usize,
}

impl Reader {
    fn new(tokens: Vec<String>) -> Self {
        return Reader {
            tokens,
            position: 0,
        };
    }
    fn next(&mut self) -> &String {
        self.position += 1;
        &self.tokens[self.position - 1]
    }
    fn peek(&self) -> Option<&String> {
        if self.tokens.len() > self.position {
            Some(&self.tokens[self.position])
        } else {
            None
        }
    }
}

pub fn read_str(code: &str) -> MalType {
    let tokens = tokenize(code);
    let mut reader = Reader::new(tokens);
    return read_form(&mut reader);
}

fn tokenize(code: &str) -> Vec<String> {
    static RE: OnceCell<Regex> = OnceCell::new();
    let re = RE.get_or_init(|| {
        Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#)
            .unwrap()
    });
    let mut res = vec![];
    for cap in re.captures_iter(code) {
        if cap[1].starts_with(";") {
            continue;
        }
        res.push(String::from(&cap[1]));
    }
    res
}

fn is_int(string: &String) -> bool {
    static INT_RE: OnceCell<Regex> = OnceCell::new();
    let re = INT_RE.get_or_init(|| Regex::new(r#"-?[0-9]+$"#).unwrap());
    re.is_match(string)
}

fn read_form(reader: &mut Reader) -> MalType {
    if let Some(token) = reader.peek() {
        match token.chars().nth(0).unwrap() {
            '(' => return read_list(reader),
            '[' => return read_vector(reader),
            _ => read_atom(reader),
        }
    } else {
        eprintln!("empty code");
        std::process::exit(1)
    }
}

fn read_list(reader: &mut Reader) -> MalType {
    match read_container_elm(reader, '(', ')') {
        Ok(ast) => MalList { elm: ast },
        Err(e) => MalError(e),
    }
}

fn read_vector(reader: &mut Reader) -> MalType {
    match read_container_elm(reader, '[', ']') {
        Ok(ast) => MalVector { elm: ast },
        Err(e) => MalError(e),
    }
}

fn read_container_elm(
    reader: &mut Reader,
    left: char,
    right: char,
) -> Result<Vec<MalType>, String> {
    let start = reader.next();
    if start.chars().nth(0).unwrap() != left {
        return Err(format!("expected {} but not", left));
    }
    let mut ast: Vec<MalType> = vec![];
    let mut token = reader.peek().unwrap();
    while token.chars().nth(0).unwrap() != right {
        ast.push(read_form(reader));
        if let Some(tk) = reader.peek() {
            token = tk;
        } else {
            return Err(format!("expected {} but not", right));
        }
    }
    reader.next();
    return Ok(ast);
}

// number | bool | nil | symbol | keyword
fn read_atom(reader: &mut Reader) -> MalType {
    let token = reader.next();
    if is_int(&token) {
        let mut r = 0;
        for c in token.chars() {
            r *= 10;
            if let Some(d) = c.to_digit(10) {
                r += d;
            } else {
                unreachable!()
            }
        }
        return MalNumber(r as i32);
    } else {
        match token.as_str() {
            "true" => MalBool(true),
            "false" => MalBool(false),
            "nil" => MalNil,
            _ => match token.chars().nth(0).unwrap() {
                ':' => MalKeyword(format!("\u{029e}{}", &token[1..])),
                _ => MalSymbol(String::from(token)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::tokenize;
    use super::MalType::{MalList, MalNumber, MalSymbol};
    use super::Reader;
    use super::{read_atom, read_form, read_list};

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize("(a, b)"), vec!["(", "a", "b", ")"]);
        assert_eq!(tokenize("(a, b);(c,d,e)"), vec!["(", "a", "b", ")"]);
        assert_eq!(tokenize("(+ a, b)"), vec!["(", "+", "a", "b", ")"]);
        assert_eq!(tokenize("(+ 12, 34)"), vec!["(", "+", "12", "34", ")"]);
    }

    #[test]
    fn test_read_form() {
        // (+ 1, (* 2, 3))
        let mut reader = Reader {
            tokens: vec![
                String::from("("),
                String::from("+"),
                String::from("1"),
                String::from("("),
                String::from("*"),
                String::from("2"),
                String::from("3"),
                String::from(")"),
                String::from(")"),
            ],
            position: 0,
        };
        assert_eq!(
            read_form(&mut reader),
            MalList {
                elm: vec![
                    MalSymbol(String::from("+")),
                    MalNumber(1),
                    MalList {
                        elm: vec![MalSymbol(String::from("*")), MalNumber(2), MalNumber(3)]
                    }
                ]
            }
        )
    }

    #[test]
    fn test_read_list() {
        let mut reader = Reader::new(vec![
            String::from("("),
            String::from("1"),
            String::from("2"),
            String::from(")"),
        ]);
        assert_eq!(
            read_list(&mut reader),
            MalList {
                elm: vec![MalNumber(1), MalNumber(2)]
            }
        )
    }

    #[test]
    fn test_read_atom() {
        let mut reader = Reader {
            tokens: vec![String::from("12"), String::from("x")],
            position: 0,
        };
        assert_eq!(read_atom(&mut reader), MalNumber(12));
        assert_eq!(read_atom(&mut reader), MalSymbol(String::from("x")));
    }
}
