fn READ(s: &str) -> &str {
    return s;
}

fn EVAL(s: &str) -> &str {
    return s;
}

fn PRINT(s: &str) -> &str {
    return s;
}

pub fn rep(s: &str) -> &str {
    return PRINT(EVAL(READ(s)));
}
