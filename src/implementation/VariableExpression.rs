use crate::enums::LiteralValue::LiteralValue;
use crate::implementation::Token::Token;
use crate::traits::Expression::Expression;
use crate::traits::Interpreter::InterpreterTrait;
use std::any::Any;

pub struct VariableExpression {
    pub variable: Token,
}

impl Clone for VariableExpression {
    fn clone(&self) -> Self {
        VariableExpression {
            variable: self.variable.clone(),
        }
    }
}

impl Expression for VariableExpression {
    fn expression_print(&self) -> String {
        return String::from("Visit variable expression");
    }

    fn interpret(
        &mut self,
        interpreter: &mut dyn InterpreterTrait,
    ) -> Result<LiteralValue, String> {
        return interpreter.visit_variable_expression(&self);
    }
    fn resolve(&mut self, resolver: &mut super::Resolver::Resolver) {
        resolver.visit_variable_expression(self);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}
