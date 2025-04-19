use std::any::{Any, TypeId};
use std::io::{self, Write};
use std::result::Result::{Err, Ok};

use crate::enums::LiteralValue::LiteralValue;
use crate::enums::TokenType::TokenType;
use crate::implementation::BinaryExpression::BinaryExpression;
use crate::implementation::Grouping::Grouping;
use crate::implementation::Literal::Literal;
use crate::implementation::Token::Token;
use crate::implementation::UnaryExpression::UnaryExpression;
use crate::traits::Expression::Expression;
use crate::traits::Statement::Statement;

use super::AssignmentExpression::AssignmentExpression;
use super::ExpressionStatement::ExpressionStatement;
use super::PrintStatement::PrintStatement;
use super::VariableExpression::VariableExpression;
use super::VariableStatement::VariableStatement;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    fn match_tokens(&mut self, token_types: &Vec<TokenType>) -> Result<bool, String> {
        for token_type in token_types {
            if self.check(*token_type)? {
                self.advance();
                return Ok(true);
            }
        }
        return Ok(false);
    }

    fn check(&self, token_type: TokenType) -> Result<bool, String> {
        let token = self.peek()?;
        return Ok(token.token_type == token_type);
    }

    fn previous(&self) -> Result<Token, String> {
        match self.tokens.get(self.current - 1) {
            Some(token) => Ok(token.clone()),
            None => Err(String::from("Token not found")),
        }
    }

    fn peek(&self) -> Result<Token, String> {
        match self.tokens.get(self.current) {
            Some(token) => Ok(token.clone()),
            None => Err(String::from("Token not found")),
        }
    }

    fn is_at_end(&self) -> Result<bool, String> {
        match self.peek() {
            Ok(token) => Ok(token.token_type == TokenType::EOF),
            Err(err) => Err(err),
        }
    }

    fn advance(&mut self) -> Result<Token, String> {
        if !self.is_at_end()? {
            self.current += 1;
        }
        return self.previous();
    }
    fn synchronize(&mut self) -> Result<(), String> {
        self.advance();
        while !self.is_at_end()? {
            if self.previous()?.token_type == TokenType::SEMICOLON {
                return Ok(());
            }
            match self.peek()?.token_type {
                TokenType::CLASS => return Ok(()),
                TokenType::FUN => return Ok(()),
                TokenType::VAR => return Ok(()),
                TokenType::FOR => return Ok(()),
                TokenType::IF => return Ok(()),
                TokenType::WHILE => return Ok(()),
                TokenType::PRINT => return Ok(()),
                TokenType::RETURN => return Ok(()),
                _ => {}
            }
            self.advance();
        }
        return Ok(());
    }

    fn error(&mut self, token: Token, message: String) {
        match token.token_type {
            TokenType::EOF => writeln!(
                io::stderr(),
                "[line {}] Error at end: {}",
                token.line,
                message
            )
            .unwrap(),
            _ => writeln!(
                io::stderr(),
                "[line {}] Error at '{}': {}",
                token.line,
                token.token_value,
                message
            )
            .unwrap(),
        }
        self.synchronize();
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Result<Token, String> {
        if self.check(token_type)? {
            return self.advance();
        }

        self.error(self.peek()?, message);
        return Err("An error occurred while consuming".to_string());
    }

    fn primary(&mut self) -> Result<Box<dyn Expression>, String> {
        if self.match_tokens(&[TokenType::FALSE].to_vec())? {
            return Ok(Box::new(Literal {
                value: LiteralValue::Boolean(false),
            }));
        } else if self.match_tokens(&[TokenType::TRUE].to_vec())? {
            return Ok(Box::new(Literal {
                value: LiteralValue::Boolean(true),
            }));
        } else if self.match_tokens(&[TokenType::NIL].to_vec())? {
            return Ok(Box::new(Literal {
                value: LiteralValue::Nil,
            }));
        } else if self.match_tokens(&[TokenType::STRING].to_vec())? {
            let token = self.previous()?;
            return Ok(Box::new(Literal {
                value: LiteralValue::String(token.token_value),
            }));
        } else if self.match_tokens(&[TokenType::NUMBER].to_vec())? {
            let token = self.previous()?;
            return Ok(Box::new(Literal {
                value: LiteralValue::Number(token.token_value),
            }));
        } else if self.match_tokens(&[TokenType::LEFT_PAREN].to_vec())? {
            match self.expression() {
                Ok(expression) => {
                    match self.consume(
                        TokenType::RIGHT_PAREN,
                        "Expect ')' after expression.".to_string(),
                    ) {
                        Ok(token) => Ok(Box::new(Grouping { expression })),
                        Err(error) => Err(error),
                    }
                }
                Err(err) => Err(err),
            }
        } else if self.match_tokens(&Vec::from([TokenType::IDENTIFIER]))? {
            let token = self.previous()?;
            return Ok(Box::new(VariableExpression { variable: token }));
        } else {
            let token = self.peek()?;
            self.error(token, "Expect expression.".to_string());
            return Err("Expect expression.".to_string());
        }
    }

    fn unary(&mut self) -> Result<Box<dyn Expression>, String> {
        if self.match_tokens(&[TokenType::BANG, TokenType::MINUS].to_vec())? {
            let operator = self.previous()?;
            match self.unary() {
                Ok(right) => {
                    return Ok(Box::new(UnaryExpression {
                        operator: operator,
                        expression: right,
                    }))
                }
                Err(error) => return Err(error),
            }
        }
        return self.primary();
    }

    fn factor(&mut self) -> Result<Box<dyn Expression>, String> {
        match self.unary() {
            Ok(mut expression) => {
                while self.match_tokens(&[TokenType::SLASH, TokenType::STAR].to_vec())? {
                    let operator = self.previous()?;
                    match self.unary() {
                        Ok(right) => {
                            expression = Box::new(BinaryExpression {
                                left: expression,
                                operator,
                                right,
                            })
                        }
                        Err(error) => return Err(error),
                    }
                }
                return Ok(expression);
            }
            Err(err) => Err(err),
        }
    }

    fn term(&mut self) -> Result<Box<dyn Expression>, String> {
        match self.factor() {
            Ok(mut expression) => {
                while self.match_tokens(&[TokenType::MINUS, TokenType::PLUS].to_vec())? {
                    let operator = self.previous()?;
                    match self.factor() {
                        Ok(right) => {
                            expression = Box::new(BinaryExpression {
                                left: expression,
                                operator,
                                right,
                            })
                        }
                        Err(err) => return Err(err),
                    }
                }
                return Ok(expression);
            }
            Err(err) => Err(err),
        }
    }

    fn comparison(&mut self) -> Result<Box<dyn Expression>, String> {
        match self.term() {
            Ok(mut expression) => {
                while self.match_tokens(
                    &[
                        TokenType::LESS,
                        TokenType::LESS_EQUAL,
                        TokenType::GREATER,
                        TokenType::GREATER_EQUAL,
                    ]
                    .to_vec(),
                )? {
                    let operator = self.previous()?;
                    match self.term() {
                        Ok(right) => {
                            expression = Box::new(BinaryExpression {
                                left: expression,
                                operator,
                                right,
                            })
                        }
                        Err(err) => return Err(err),
                    }
                }
                return Ok(expression);
            }
            Err(err) => Err(err),
        }
    }

    fn equality(&mut self) -> Result<Box<dyn Expression>, String> {
        match self.comparison() {
            Ok(mut expression) => {
                while self
                    .match_tokens(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL].to_vec())?
                {
                    let operator = self.previous()?;
                    match self.comparison() {
                        Ok(right) => {
                            expression = Box::new(BinaryExpression {
                                left: expression,
                                operator,
                                right,
                            })
                        }
                        Err(err) => return Err(err),
                    }
                }

                return Ok(expression);
            }
            Err(err) => Err(err),
        }
    }

    fn assignment(&mut self) -> Result<Box<dyn Expression>, String> {
        let expression = self.equality()?;
        if self.match_tokens(&Vec::from([TokenType::EQUAL]))? {
            let equals = self.previous()?;
            let value = self.assignment()?;

            if let Some(var_expr) = expression.as_any().downcast_ref::<VariableExpression>() {
                let name = var_expr.variable.clone();
                return Ok(Box::new(AssignmentExpression { name, value }));
            }
            self.error(equals.clone(), String::from("Invalid assignment target."))
        }
        return Ok(expression);
    }

    pub fn expression(&mut self) -> Result<Box<dyn Expression>, String> {
        return self.assignment();
    }

    fn expression_statement(&mut self) -> Result<ExpressionStatement, String> {
        let expression = self.expression()?;
        match self.consume(
            TokenType::SEMICOLON,
            String::from("Expect ';' after value."),
        ) {
            Ok(_) => Ok(ExpressionStatement { expression }),
            Err(error) => Err(error),
        }
    }

    fn print_statement(&mut self) -> Result<PrintStatement, String> {
        let expression = self.expression()?;
        match self.consume(
            TokenType::SEMICOLON,
            String::from("Expect ';' after value."),
        ) {
            Ok(_) => Ok(PrintStatement { expression }),
            Err(error) => Err(error),
        }
    }

    fn statement(&mut self) -> Result<Box<dyn Statement>, String> {
        if self.match_tokens(&vec![TokenType::PRINT])? {
            let stmt = self.print_statement()?;
            Ok(Box::new(stmt))
        } else {
            let stmt = self.expression_statement()?;
            Ok(Box::new(stmt))
        }
    }

    fn var_declaration(&mut self) -> Result<Box<dyn Statement>, String> {
        let name = self.consume(TokenType::IDENTIFIER, String::from("Expect variable name."))?;

        let mut initializer = None;
        if self.match_tokens(&Vec::from([TokenType::EQUAL]))? {
            initializer = Some(self.expression()?);
        }
        let _ = self.consume(
            TokenType::SEMICOLON,
            String::from("Expect ';' after variable declaration."),
        );
        match initializer {
            Some(v) => Ok(Box::new(VariableStatement {
                initializer: v,
                name,
            })),
            None => Ok(Box::new(VariableStatement {
                initializer: Box::new(Literal {
                    value: LiteralValue::Nil,
                }),
                name,
            })),
        }
    }

    fn declaration(&mut self) -> Result<Box<dyn Statement>, String> {
        if self.match_tokens(&Vec::from([TokenType::VAR]))? {
            return self.var_declaration();
        }
        return self.statement();
    }

    pub fn parse(&mut self) -> Result<Vec<Box<dyn Statement>>, String> {
        let mut statements = Vec::new();
        while !self.is_at_end()? {
            statements.push(self.declaration()?);
        }

        return Ok(statements);
    }
}
