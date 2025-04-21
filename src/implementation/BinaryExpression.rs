use crate::enums::LiteralValue::LiteralValue;
use crate::implementation::AstPrinter::AstPrinter;
use crate::implementation::Token::Token;
use crate::traits::AstPrinter::AstPrinterTrait;
use crate::traits::Expression::Expression;
use crate::traits::Interpreter::InterpreterTrait;
use std::any::Any;

pub struct BinaryExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: Token,
}

impl Clone for BinaryExpression {
    fn clone(&self) -> Self {
        BinaryExpression {
            left: self.left.clone_box(),
            right: self.right.clone_box(),
            operator: self.operator.clone(),
        }
    }
}

impl Expression for BinaryExpression {
    fn expression_print(&self) -> String {
        let visitor = AstPrinter {};
        return visitor.visit_binary_expression(self);
    }
    fn interpret(
        &mut self,
        interpreter: &mut dyn InterpreterTrait,
    ) -> Result<LiteralValue, String> {
        return interpreter.visit_binary_expression(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}
