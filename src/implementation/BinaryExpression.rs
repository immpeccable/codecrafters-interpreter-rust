use crate::implementation::AstPrinter::AstPrinter;
use crate::implementation::Token::Token;
use crate::traits::Expression::Expression;
use crate::traits::ExpressionVisitor::ExpressionVisitor;

pub struct BinaryExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: Token,
}

impl Expression for BinaryExpression {
    fn accept(&self) -> String {
        let visitor = AstPrinter {};
        return visitor.visit_binary_expression(self);
    }
}
