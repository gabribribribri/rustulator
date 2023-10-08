pub const NUMBER_PTY: i32 = 0;
pub const ADDSUB_PTY: i32 = 0;

pub fn symbols(c: char) -> bool {
    if digit(c) {
        return true;
    }
    if operators(c) {
        return true;
    }
    c == ' '
}

pub fn plusminus(c: char) -> bool {
    c == '+' || c == '-'
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

pub fn digit(c: char) -> bool {
    ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(&c)
}

pub fn operators(c: char) -> bool {
    ['+', '-', '*', '/', '^', '(', ')'].contains(&c)
}
