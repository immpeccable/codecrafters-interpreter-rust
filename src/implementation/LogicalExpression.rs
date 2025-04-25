use crate::enums::LiteralValue::LiteralValue;
use crate::implementation::Token::Token;
use crate::traits::Expression::Expression;
use crate::traits::Interpreter::InterpreterTrait;
use std::any::Any;

pub struct LogicalExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: Token,
}

impl Clone for LogicalExpression {
    fn clone(&self) -> Self {
        LogicalExpression {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
            operator: self.operator.clone(),
        }
    }
}

impl Expression for LogicalExpression {
    fn expression_print(&self) -> String {
        return String::from("zz");
    }

    fn resolve(&mut self, resolver: &mut super::Resolver::Resolver) {
        resolver.visit_logical_expression(self);
    }
    fn interpret(
        &mut self,
        interpreter: &mut dyn InterpreterTrait,
    ) -> Result<LiteralValue, String> {
        return interpreter.visit_logical_expression(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}
