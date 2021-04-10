use crate::token::Token;
#[derive(Debug, PartialEq)]
pub enum Expr {
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Token),
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        use Expr::*;
        String::from(match self {
            Unary(_, _) => "Unary",
            Binary(_, _, _) => "Binary",
            Grouping(_) => "Grouping",
            Literal(_) => "Literal",
        })
    }
}

pub trait Visitor<T> {
    fn new() -> Self;

    fn visit_expr(&mut self, expr: &Expr) -> T {
        use Expr::*;

        match expr {
            Binary(left, token, right) => self.visit_expr_binary(left, token, right),
            Unary(token, expression) => self.visit_expr_unary(token, expression),
            Grouping(expression) => self.visit_expr_grouping(expression),
            Literal(token) => self.visit_expr_literal(token),
        }
    }

    fn visit_expr_unary(&mut self, _token: &Token, _expr: &Expr) -> T {
        unimplemented!()
    }

    fn visit_expr_binary(&mut self, _left: &Expr, _token: &Token, _right: &Expr) -> T {
        unimplemented!()
    }

    fn visit_expr_grouping(&mut self, _expr: &Expr) -> T {
        unimplemented!()
    }

    fn visit_expr_literal(&mut self, _token: &Token) -> T {
        unimplemented!()
    }
}

pub struct VisitorPrinter {}

impl VisitorPrinter {
    pub fn print(&mut self, expr: &Expr) -> String {
        self.visit_expr(expr)
    }

    fn parenthesize(&mut self, output: String) -> String {
        format!("({})", output)
    }
}

impl Visitor<String> for VisitorPrinter {
    fn new() -> Self {
        VisitorPrinter {}
    }

    fn visit_expr_binary(
        &mut self,
        left: &Expr,
        token: &Token,
        right: &Expr,
    ) -> String {
        let left_print = self.visit_expr(&left);
        let right_print = self.visit_expr(&right);
        self.parenthesize(format!(
            "{} {} {}",
            token.to_string(),
            left_print,
            right_print
        ))
    }

    fn visit_expr_grouping(&mut self, expr: &Expr) -> String {
        let printed_expr = self.visit_expr(&expr);
        self.parenthesize(format!("group {}", printed_expr))
    }

    fn visit_expr_literal(&mut self, token: &Token) -> String {
        format!("{}", token.to_string())
    }

    fn visit_expr_unary(&mut self, token: &Token, expr: &Expr) -> String {
        let printed_expr = self.visit_expr(&expr);
        self.parenthesize(format!("{} {}", token.to_string(), printed_expr))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn print_visit() {
        use crate::expr::{Expr, VisitorPrinter};
        use crate::token::Token;
        use Expr::*;
        let mut visitor = VisitorPrinter {};
        let node = Binary(
            Box::new(Unary(Token::Minus, Box::new(Literal(Token::Number(123.0))))),
            Token::Star,
            Box::new(Grouping(Box::new(Literal(Token::Number(45.67))))),
        );
        let string_ast = VisitorPrinter::print(&mut visitor, &node);

        assert_eq!("(* (- 123) (group 45.67))", string_ast);
    }
}
