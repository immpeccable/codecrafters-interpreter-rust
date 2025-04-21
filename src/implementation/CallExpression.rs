use crate::enums::LiteralValue::LiteralValue;
use crate::implementation::AstPrinter::AstPrinter;
use crate::implementation::Token::Token;
use crate::traits::Expression::Expression;
use crate::traits::Interpreter::InterpreterTrait;
use std::any::Any;

pub struct CallExpression {
    pub callee: Box<dyn Expression>,
    pub paren: Token,
    pub arguments: Vec<Box<dyn Expression>>,
}

impl Expression for CallExpression {
    fn expression_print(&self) -> String {
        return String::from("zzz");
    }
    fn interpret(
        &mut self,
        interpreter: &mut dyn InterpreterTrait,
    ) -> Result<LiteralValue, String> {
        return interpreter.visit_call_expression(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
