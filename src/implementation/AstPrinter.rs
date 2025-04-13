use crate::implementation::BinaryExpression::BinaryExpression;
use crate::implementation::Grouping::Grouping;
use crate::implementation::Literal::Literal;
use crate::implementation::UnaryExpression::UnaryExpression;
use crate::traits::AstPrinter::AstPrinterTrait;
use crate::traits::Expression::Expression;

pub struct AstPrinter {}

impl AstPrinterTrait for AstPrinter {
    fn print(&self, expression: &Box<dyn Expression>) -> String {
        return expression.expression_print();
    }
    fn visit_binary_expression(&self, expression: &BinaryExpression) -> String {
        return self.paranthesize(
            expression.operator.token_value.clone(),
            &[&expression.left, &expression.right].to_vec(),
        );
    }

    fn visit_literal(&self, expression: &Literal) -> String {
        return expression.value.to_string();
    }
    fn visit_grouping(&self, expression: &Grouping) -> String {
        return self.paranthesize("group".to_string(), &[&expression.expression].to_vec());
    }

    fn visit_unary_expression(&self, expression: &UnaryExpression) -> String {
        return self.paranthesize(
            expression.operator.token_value.clone(),
            &[&expression.expression].to_vec(),
        );
    }

    fn paranthesize(&self, name: String, expressions: &Vec<&Box<dyn Expression>>) -> String {
        let mut result = String::new();
        result.push_str("(");
        result.push_str(&name);
        for expression in expressions {
            result.push_str(" ");
            result.push_str(expression.expression_print().as_str());
        }
        result.push_str(")");
        return result;
    }
}
