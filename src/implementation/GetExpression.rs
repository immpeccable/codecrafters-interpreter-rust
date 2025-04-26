use crate::{enums::LiteralValue::LiteralValue, traits::Expression::Expression};

use super::Token::Token;

pub struct GetExpression {
    pub expression: Box<dyn Expression>,
    pub name: Token,
    pub id: u32,
}

impl Clone for GetExpression {
    fn clone(&self) -> Self {
        GetExpression {
            name: self.name.clone(),
            expression: self.expression.clone_box(),
            id: self.id,
        }
    }
}

impl Expression for GetExpression {
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
        resolver.visit_get_expression(self);
    }
    fn interpret(
        &mut self,
        interpreter: &mut dyn crate::traits::Interpreter::InterpreterTrait,
    ) -> Result<LiteralValue, String> {
        return interpreter.visit_get_expression(self);
    }
}
