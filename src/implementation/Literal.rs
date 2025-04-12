use crate::enums::LiteralValue::LiteralValue;
use crate::implementation::AstPrinter::AstPrinter;
use crate::traits::Expression::Expression;
use crate::traits::ExpressionVisitor::ExpressionVisitor;

pub struct Literal {
    pub value: LiteralValue,
}

impl Expression for Literal {
    fn accept(&self) -> String {
        let visitor = AstPrinter {};
        return visitor.visit_literal(self);
    }
}
