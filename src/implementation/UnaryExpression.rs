use std::any::Any;

use crate::enums::LiteralValue::LiteralValue;
use crate::implementation::AstPrinter::AstPrinter;
use crate::implementation::Token::Token;
use crate::traits::AstPrinter::AstPrinterTrait;
use crate::traits::Expression::Expression;
use crate::traits::Interpreter::InterpreterTrait;

pub struct UnaryExpression {
    pub operator: Token,
    pub expression: Box<dyn Expression>,
}

impl Clone for UnaryExpression {
    fn clone(&self) -> Self {
        UnaryExpression {
            operator: self.operator.clone(),
            expression: self.expression.clone_box(),
        }
    }
}

impl Expression for UnaryExpression {
    fn expression_print(&self) -> String {
        let visitor = AstPrinter {};
        return visitor.visit_unary_expression(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn interpret(
        &mut self,
        interpreter: &mut dyn InterpreterTrait,
    ) -> Result<LiteralValue, String> {
        return interpreter.visit_unary_expression(self);
    }

    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}
