mod errors;
mod rules;

pub struct Expression<'a> {
    expr: &'a str,
}



impl<'a> Expression<'a> {

    pub fn evalutate(&self) -> Result<f64, errors::Error>{
        if let Err(e) = self.verify() {
            return Err(e);
        }

        return Expression::resolve(self.expr);
    }

    pub fn verify(&self) -> Result<(), errors::Error> {
        let mut left_parentheses = 0;
        let mut right_parentheses = 0;

        for (i, c) in self.expr.chars().enumerate() {
            match c {
                '(' => {
                        left_parentheses += 1;
                        if let Some(c) = self.ch(i + 1) {
                        if c == ')' {
                            return Err(Error::Syntax(SyntaxError::VoidParentheses));
                        }
                    }
                }

                ')' => {
                    right_parentheses += 1;
                    if right_parentheses > left_parentheses {
                        return Err(Error::Syntax(SyntaxError::ParenthesesPlacement));
                    }
                }

                '+' | '-' => {
                    match self.ch(i + 1) {
                        Some(right) => {
                            if !rules::right_oper(right) {
                                return Err(Error::Syntax(SyntaxError::EntangledOperators));
                            }
                        },
                        None => return Err(Error::Syntax(SyntaxError::EntangledOperators))
                    }
                },
                
                '*' | '/' | '^' => {
                    match (self.ch(i - 1), self.ch(i + 1)) {
                        (Some(left), Some(right)) => {
                            if !rules::left_oper(left) || !rules::right_oper(right) {
                                return Err(Error::Syntax(SyntaxError::EntangledOperators));
                            }
                        },
                        _ => return Err(Error::Syntax(SyntaxError::EntangledOperators)),
                    }
                }

                _ => {
                    if !rules::symbols(c) {
                        return Err(Error::Syntax(SyntaxError::UnknownSymbol));
                    }
                }
            }
        }

        if left_parentheses != right_parentheses {
            return Err(Error::Syntax(SyntaxError::MissingParentheses));
        }

        Ok(())
    }

    pub fn resolve(expr: &str) -> Result<f64, errors::Error> {

        
        todo!()
    }

    fn ch(&self, i: usize) -> Option<char> {
        self.expr.chars().nth(i)
    }

    pub fn new(expr: &'a str) -> Self {
        Self { expr }
    }
}