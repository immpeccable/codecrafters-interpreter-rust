use crate::implementation::BinaryExpression::BinaryExpression;
use crate::implementation::Grouping::Grouping;
use crate::implementation::Literal::Literal;
use crate::implementation::UnaryExpression::UnaryExpression;
use crate::implementation::VariableExpression::VariableExpression;
use crate::traits::Expression::Expression;

pub trait AstPrinterTrait {
    fn visit_binary_expression(&self, expression: &BinaryExpression) -> String;
    fn visit_unary_expression(&self, expression: &UnaryExpression) -> String;
    fn visit_grouping(&self, expression: &Grouping) -> String;
    fn visit_literal(&self, expression: &Literal) -> String;
    fn paranthesize(&self, name: String, expression: &Vec<&Box<dyn Expression>>) -> String;
}
