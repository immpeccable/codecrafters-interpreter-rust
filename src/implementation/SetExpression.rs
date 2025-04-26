use crate::{enums::LiteralValue::LiteralValue, traits::Expression::Expression};

use super::Token::Token;

pub struct SetExpression {
    pub expression: Box<dyn Expression>,
    pub name: Token,
    pub value: Box<dyn Expression>,
    pub id: u32,
}

impl Clone for SetExpression {
    fn clone(&self) -> Self {
        SetExpression {
            name: self.name.clone(),
            expression: self.expression.clone_box(),
            value: self.value.clone_box(),
            id: self.id,
        }
    }
} 

impl Expression for SetExpression {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
    fn expression_print(&self) -> String {
        String::from("zz")
    }
    fn id(&self) -> u32 {
        self.id
    }
    fn resolve(&mut self, resolver: &mut super::Resolver::Resolver) {
        resolver.visit_set_expression(self);
    }
    fn interpret(
        &mut self,
        interpreter: &mut dyn crate::traits::Interpreter::InterpreterTrait,
    ) -> Result<LiteralValue, String> {
        return interpreter.visit_set_expression(self);
    }
}
