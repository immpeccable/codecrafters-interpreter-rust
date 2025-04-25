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
    pub id: u32,
}

impl Clone for CallExpression {
    fn clone(&self) -> Self {
        CallExpression {
            callee: self.callee.clone_box(),
            paren: self.paren.clone(),
            arguments: self.arguments.iter().map(|arg| arg.clone_box()).collect(),
            id: self.id,
        }
    }
}

impl Expression for CallExpression {
    fn expression_print(&self) -> String {
        return String::from("zzz");
    }

    fn id(&self) -> u32 {
        self.id
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

    fn resolve(&mut self, resolver: &mut super::Resolver::Resolver) {
        resolver.visit_call_expression(self);
    }

    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}
