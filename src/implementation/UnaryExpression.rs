use crate::implementation::AstPrinter::AstPrinter;
use crate::implementation::Token::Token;
use crate::traits::Expression::Expression;
use crate::traits::ExpressionVisitor::ExpressionVisitor;
pub struct UnaryExpression {
    pub operator: Token,
    pub expression: Box<dyn Expression>,
}

impl Expression for UnaryExpression {
    fn accept(&self) -> String {
        let visitor = AstPrinter {};
        return visitor.visit_unary_expression(self);
    }
}
