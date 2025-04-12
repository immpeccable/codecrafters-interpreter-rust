use crate::implementation::AstPrinter::AstPrinter;
use crate::implementation::Token::Token;
use crate::traits::Expression::Expression;
use crate::traits::ExpressionVisitor::ExpressionVisitor;

pub struct Grouping {
    pub expression: Box<dyn Expression>,
}

impl Expression for Grouping {
    fn accept(&self) -> String {
        let visitor = AstPrinter {};
        return visitor.visit_grouping(self);
    }
}
