use crate::{enums::LiteralValue::LiteralValue, traits::Expression::Expression};

use super::Token::Token;

pub struct SuperExpression {
    pub keyword: Token,
    pub method: Token,
    pub id: u32,
}

impl Clone for SuperExpression {
    fn clone(&self) -> Self {
        SuperExpression {
            keyword: self.keyword.clone(),
            method: self.method.clone(),
            id: self.id,
        }
    }
}

impl Expression for SuperExpression {
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
        resolver.visit_super_expression(self);
    }
    fn interpret(
        &mut self,
        interpreter: &mut dyn crate::traits::Interpreter::InterpreterTrait,
    ) -> Result<LiteralValue, String> {
        return interpreter.visit_super_expression(self);
    }
}
