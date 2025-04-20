use crate::enums::LiteralValue::LiteralValue;
use crate::implementation::AssignmentExpression::AssignmentExpression;
use crate::implementation::BinaryExpression::BinaryExpression;
use crate::implementation::BlockStatement::BlockStatement;
use crate::implementation::ExpressionStatement::ExpressionStatement;
use crate::implementation::Grouping::Grouping;
use crate::implementation::IfStatement::IfStatement;
use crate::implementation::Literal::Literal;
use crate::implementation::LogicalExpression::LogicalExpression;
use crate::implementation::PrintStatement::PrintStatement;
use crate::implementation::Token::Token;
use crate::implementation::UnaryExpression::UnaryExpression;
use crate::implementation::VariableExpression::VariableExpression;
use crate::implementation::VariableStatement::VariableStatement;
use crate::implementation::WhileStatement::WhileStatement;
use crate::traits::Expression::Expression;

use super::Statement::Statement;

pub trait InterpreterTrait {
    fn visit_binary_expression(
        &mut self,
        expression: &mut BinaryExpression,
    ) -> Result<LiteralValue, String>;
    fn visit_unary_expression(
        &mut self,
        expression: &mut UnaryExpression,
    ) -> Result<LiteralValue, String>;
    fn visit_logical_expression(
        &mut self,
        expression: &mut LogicalExpression,
    ) -> Result<LiteralValue, String>;
    fn visit_print_statement(&mut self, statement: &mut PrintStatement);
    fn visit_expression_statement(&mut self, statement: &mut ExpressionStatement);
    fn visit_variable_statement(&mut self, statement: &mut VariableStatement);
    fn visit_grouping(&mut self, expression: &mut Grouping) -> Result<LiteralValue, String>;
    fn visit_literal(&self, expression: &Literal) -> Result<LiteralValue, String>;
    fn evaluate(&mut self, expression: &mut Box<dyn Expression>) -> Result<LiteralValue, String>;
    fn is_truthy(&self, expression: &LiteralValue) -> bool;
    fn is_equal(&self, left: &LiteralValue, right: &LiteralValue) -> bool;
    fn execute(&mut self, statement: &mut Box<dyn Statement>);
    fn error(&self, message: String, token: &Token) -> String;
    fn interpret(&mut self, statements: &mut Vec<Box<dyn Statement>>);
    fn visit_variable_expression(
        &mut self,
        expression: &VariableExpression,
    ) -> Result<LiteralValue, String>;
    fn visit_assignment_expression(
        &mut self,
        expression: &mut AssignmentExpression,
    ) -> Result<LiteralValue, String>;
    fn visit_block_statement(&mut self, statement: &mut BlockStatement);
    fn visit_if_statement(&mut self, statement: &mut IfStatement) -> Result<(), String>;
    fn visit_while_statement(&mut self, statement: &mut WhileStatement) -> Result<(), String>;
}
