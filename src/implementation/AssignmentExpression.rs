use crate::enums::LiteralValue::LiteralValue;
use crate::implementation::Token::Token;
use crate::traits::Expression::Expression;
use crate::traits::Interpreter::InterpreterTrait;
use std::any::Any;

pub struct AssignmentExpression {
    pub name: Token,
    pub value: Box<dyn Expression>,
    pub id: u32,
}

impl Clone for AssignmentExpression {
    fn clone(&self) -> Self {
        AssignmentExpression {
            name: self.name.clone(),
            value: self.value.clone_box(),
            id: self.id,
        }
    }
}

impl Expression for AssignmentExpression {
    fn resolve(&mut self, resolver: &mut super::Resolver::Resolver) {
        resolver.visit_assignment_expression(self);
    }

    fn id(&self) -> u32 {
        self.id
    }
    fn expression_print(&self) -> String {
        return String::from("Visit variable expression");
    }

    fn interpret(
        &mut self,
        interpreter: &mut dyn InterpreterTrait,
    ) -> Result<LiteralValue, String> {
        return interpreter.visit_assignment_expression(self);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}
