use crate::{
    enums::LiteralValue::LiteralValue,
    traits::{Expression::Expression, Interpreter::InterpreterTrait, Statement::Statement},
};

pub struct PrintStatement {
    pub expression: Box<dyn Expression>,
}

impl Clone for PrintStatement {
    fn clone(&self) -> Self {
        PrintStatement {
            expression: self.expression.clone_box(),
        }
    }
}

impl Statement for PrintStatement {
    fn interpret(
        &mut self,
        interpreter: &mut dyn InterpreterTrait,
    ) -> Result<Option<LiteralValue>, String> {
        return interpreter.visit_print_statement(self);
    }

    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}
