use std::any::Any;

use crate::enums::LiteralValue::LiteralValue;
use crate::implementation::AstPrinter::AstPrinter;
use crate::implementation::Token::Token;
use crate::traits::AstPrinter::AstPrinterTrait;
use crate::traits::Expression::Expression;
use crate::traits::Interpreter::InterpreterTrait;

use super::Interpreter::Interpreter;
pub struct UnaryExpression {
    pub operator: Token,
    pub expression: Box<dyn Expression>,
}

impl Expression for UnaryExpression {
    fn expression_print(&self) -> String {
        let visitor = AstPrinter {};
        return visitor.visit_unary_expression(self);
    }

    fn interpret(&self) -> Result<LiteralValue, String> {
        let visitor = Interpreter {};
        return visitor.visit_unary_expression(self);
    }
}
