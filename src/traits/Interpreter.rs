use crate::enums::LiteralValue::LiteralValue;
use crate::implementation::BinaryExpression::BinaryExpression;
use crate::implementation::ExpressionStatement::ExpressionStatement;
use crate::implementation::Grouping::Grouping;
use crate::implementation::Literal::Literal;
use crate::implementation::PrintStatement::PrintStatement;
use crate::implementation::Token::Token;
use crate::implementation::UnaryExpression::UnaryExpression;
use crate::traits::Expression::Expression;

use super::Statement::Statement;

pub trait InterpreterTrait {
    fn visit_binary_expression(
        &self,
        expression: &BinaryExpression,
    ) -> Result<LiteralValue, String>;
    fn visit_unary_expression(&self, expression: &UnaryExpression) -> Result<LiteralValue, String>;
    fn visit_print_statement(&self, statement: &PrintStatement);
    fn visit_expression_statement(&self, statement: &ExpressionStatement);
    fn visit_grouping(&self, expression: &Grouping) -> Result<LiteralValue, String>;
    fn visit_literal(&self, expression: &Literal) -> Result<LiteralValue, String>;
    fn evaluate(&self, expression: &Box<dyn Expression>) -> Result<LiteralValue, String>;
    fn is_truthy(&self, expression: &LiteralValue) -> bool;
    fn is_equal(&self, left: &LiteralValue, right: &LiteralValue) -> bool;
    fn execute(&self, statement: &Box<dyn Statement>);
    fn error(&self, message: String, token: &Token) -> String;
    fn interpret(&self, statements: Vec<Box<dyn Statement>>);
}
