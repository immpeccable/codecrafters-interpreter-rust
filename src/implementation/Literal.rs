use std::any::Any;

use rand::{rng, Rng};

use crate::enums::LiteralValue::LiteralValue;
use crate::implementation::AstPrinter::AstPrinter;
use crate::traits::AstPrinter::AstPrinterTrait;
use crate::traits::Expression::Expression;
use crate::traits::Interpreter::InterpreterTrait;

pub struct Literal {
    pub value: LiteralValue,
    pub id: u32,
}

impl Literal {
    /// constructor that picks a random `usize` for you
    pub fn new(value: LiteralValue) -> Self {
        let mut rng = rng();
        Literal {
            value,
            id: rng.random::<u32>(), // random usize
        }
    }
}

// if you also want to be able to do `Literal::default()`
impl Default for Literal {
    fn default() -> Self {
        // you'll need `LiteralValue: Default` here
        Literal::new(Default::default())
    }
}

impl Clone for Literal {
    fn clone(&self) -> Self {
        Literal {
            value: self.value.clone(),
            id: self.id,
        }
    }
}

impl Expression for Literal {
    fn expression_print(&self) -> String {
        let visitor = AstPrinter {};
        return visitor.visit_literal(self);
    }

    fn id(&self) -> u32 {
        self.id
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
