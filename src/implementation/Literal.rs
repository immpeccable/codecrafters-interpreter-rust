use std::any::Any;

use crate::enums::LiteralValue::LiteralValue;
use crate::implementation::AstPrinter::AstPrinter;
use crate::traits::AstPrinter::AstPrinterTrait;
use crate::traits::Expression::Expression;
use crate::traits::Interpreter::InterpreterTrait;

pub struct Literal {
    pub value: LiteralValue,
}

impl Clone for Literal {
    fn clone(&self) -> Self {
        Literal {
            value: self.value.clone(),
        }
    }
}

impl Expression for Literal {
    fn expression_print(&self) -> String {
        let visitor = AstPrinter {};
        return visitor.visit_literal(self);
    }

    fn resolve(&mut self, resolver: &mut super::Resolver::Resolver) {
        resolver.visit_literal_expression(self);
    }

    fn interpret(
        &mut self,
        interpreter: &mut dyn InterpreterTrait,
    ) -> Result<LiteralValue, String> {
        return interpreter.visit_literal(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}
