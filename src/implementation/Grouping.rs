use crate::enums::LiteralValue::LiteralValue;
use crate::implementation::AstPrinter::AstPrinter;
use crate::traits::AstPrinter::AstPrinterTrait;
use crate::traits::Expression::Expression;
use crate::traits::Interpreter::InterpreterTrait;

pub struct Grouping {
    pub expression: Box<dyn Expression>,
}

impl Expression for Grouping {
    fn expression_print(&self) -> String {
        let visitor = AstPrinter {};
        return visitor.visit_grouping(self);
    }
    fn interpret(&mut self, interpreter: &mut dyn InterpreterTrait) -> Result<LiteralValue, String> {
        return interpreter.visit_grouping(self);
    }
}
