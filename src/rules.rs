pub enum Priority {
    Number,
    Addition,
    Multiply,
    Exponant,
    Parentheses,
}

pub fn symbols(c: char) -> bool {
    if digit(c) {
        return true;
    }
    if operators(c) {
        return true;
    }
    c == ' '
}

pub fn right_oper(c: char) -> bool {
    if digit(c) {
        return true;
    }
    ['+', '-', '('].contains(&c)
}

pub fn left_oper(c: char) -> bool {
    if digit(c) {
        return true;
    }
    // [')'].contains(&c)
    c == ')'
}

pub fn letter(c: char) -> bool {
    ('a'..='z').contains(&c.to_ascii_lowercase())
}

pub fn digit(c: char) -> bool {
    ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(&c)
}

pub fn operators(c: char) -> bool {
    ['+', '-', '*', '/', '^', '(', ')'].contains(&c)
}
