use std::any::Any;

use rand::{rng, Rng};

use crate::enums::LiteralValue::LiteralValue;
use crate::implementation::AstPrinter::AstPrinter;
use crate::traits::AstPrinter::AstPrinterTrait;
use crate::traits::Expression::Expression;
use crate::traits::Interpreter::InterpreterTrait;

use super::Token::Token;

pub struct ThisExpression {
    pub value: Token,
    pub id: u32,
}

impl ThisExpression {
    /// constructor that picks a random `usize` for you
    pub fn new(value: Token) -> Self {
        let mut rng = rng();
        ThisExpression {
            value,
            id: rng.random::<u32>(), // random usize
        }
    }
}

impl Clone for ThisExpression {
    fn clone(&self) -> Self {
        ThisExpression {
            value: self.value.clone(),
            id: self.id,
        }
    }
}

impl Expression for ThisExpression {
    fn expression_print(&self) -> String {
        return String::from("zz");
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn resolve(&mut self, resolver: &mut super::Resolver::Resolver) {
        resolver.visit_this_expression(self);
    }

    fn interpret(
        &mut self,
        interpreter: &mut dyn InterpreterTrait,
    ) -> Result<LiteralValue, String> {
        return interpreter.visit_this_expression(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}
