use std::any::Any;

use crate::enums::LiteralValue::LiteralValue;
use crate::implementation::AstPrinter::AstPrinter;
use crate::traits::AstPrinter::AstPrinterTrait;
use crate::traits::Expression::Expression;
use crate::traits::Interpreter::InterpreterTrait;

pub struct Literal {
    pub value: LiteralValue,
}

impl Expression for Literal {
    fn expression_print(&self) -> String {
        let visitor = AstPrinter {};
        return visitor.visit_literal(self);
    }

    fn interpret(&mut self, interpreter: &mut dyn InterpreterTrait) -> Result<LiteralValue, String> {
        return interpreter.visit_literal(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
