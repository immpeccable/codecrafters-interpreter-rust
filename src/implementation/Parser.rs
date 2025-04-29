use std::io::{self, Write};
use std::result::Result::{Err, Ok};

use rand::{rng, Rng};

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
use super::BlockStatement::BlockStatement;
use super::CallExpression::CallExpression;
use super::ClassStatement::ClassStatement;
use super::ExpressionStatement::ExpressionStatement;
use super::FunctionStatement::FunctionStatement;
use super::GetExpression::GetExpression;
use super::IfStatement::IfStatement;
use super::LogicalExpression::LogicalExpression;
use super::PrintStatement::PrintStatement;
use super::ReturnStatement::ReturnStatement;
use super::SetExpression::SetExpression;
use super::ThisExpression::ThisExpression;
use super::VariableExpression::VariableExpression;
use super::VariableStatement::VariableStatement;
use super::WhileStatement::WhileStatement;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    fn match_tokens(&mut self, token_types: &Vec<TokenType>) -> Result<bool, String> {
        for token_type in token_types {
            if self.check(*token_type)? {
                self.advance()?;
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
        self.advance()?;
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
            self.advance()?;
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
                id: rng().random(),
            }));
        } else if self.match_tokens(&[TokenType::TRUE].to_vec())? {
            return Ok(Box::new(Literal {
                value: LiteralValue::Boolean(true),
                id: rng().random(),
            }));
        } else if self.match_tokens(&[TokenType::NIL].to_vec())? {
            return Ok(Box::new(Literal {
                value: LiteralValue::Nil,
                id: rng().random(),
            }));
        } else if self.match_tokens(&[TokenType::STRING].to_vec())? {
            let token = self.previous()?;
            return Ok(Box::new(Literal {
                value: LiteralValue::String(token.token_value),
                id: rng().random(),
            }));
        } else if self.match_tokens(&[TokenType::NUMBER].to_vec())? {
            let token = self.previous()?;
            return Ok(Box::new(Literal {
                value: LiteralValue::Number(token.token_value),
                id: rng().random(),
            }));
        } else if self.match_tokens(&[TokenType::LEFT_PAREN].to_vec())? {
            match self.expression() {
                Ok(expression) => {
                    match self.consume(
                        TokenType::RIGHT_PAREN,
                        "Expect ')' after expression.".to_string(),
                    ) {
                        Ok(_) => Ok(Box::new(Grouping {
                            expression,
                            id: rng().random(),
                        })),
                        Err(error) => Err(error),
                    }
                }
                Err(err) => Err(err),
            }
        } else if self.match_tokens(&Vec::from([TokenType::IDENTIFIER]))? {
            let token = self.previous()?;
            return Ok(Box::new(VariableExpression {
                variable: token,
                id: rng().random(),
            }));
        } else if self.match_tokens(&Vec::from([TokenType::THIS]))? {
            let token = self.previous()?;
            return Ok(Box::new(ThisExpression {
                value: token,
                id: rng().random(),
            }));
        } else {
            let token = self.peek()?;
            self.error(token, "Expect expression.".to_string());
            return Err("Expect expression.".to_string());
        }
    }

    fn finish_call(&mut self, callee: Box<dyn Expression>) -> Result<Box<dyn Expression>, String> {
        let mut arguments = Vec::new();
        if !self.check(TokenType::RIGHT_PAREN)? {
            arguments.push(self.expression()?);
            while self.match_tokens(&vec![TokenType::COMMA])? {
                if arguments.len() >= 255 {
                    self.error(
                        self.peek()?,
                        String::from("Can't have more than 255 arguments."),
                    )
                }
                arguments.push(self.expression()?);
            }
        }
        let paren = self.consume(
            TokenType::RIGHT_PAREN,
            String::from("Expect ')' after arguments."),
        )?;
        return Ok(Box::new(CallExpression {
            callee,
            paren,
            arguments,
            id: rng().random(),
        }));
    }

    fn call(&mut self) -> Result<Box<dyn Expression>, String> {
        let mut expression = self.primary()?;
        let mut i = 0;
        loop {
            if self.match_tokens(&vec![TokenType::LEFT_PAREN])? {
                expression = self.finish_call(expression)?;
            } else if self.match_tokens(&vec![TokenType::DOT])? {
                let name = self.consume(
                    TokenType::IDENTIFIER,
                    String::from("Expect property name after '.'."),
                )?;
                expression = Box::new(GetExpression {
                    expression,
                    name,
                    id: rng().random(),
                })
            } else {
                break;
            }
            i += 1
        }
        return Ok(expression);
    }

    fn unary(&mut self) -> Result<Box<dyn Expression>, String> {
        if self.match_tokens(&[TokenType::BANG, TokenType::MINUS].to_vec())? {
            let operator = self.previous()?;
            match self.unary() {
                Ok(right) => {
                    return Ok(Box::new(UnaryExpression {
                        operator,
                        expression: right,
                        id: rng().random(),
                    }))
                }
                Err(error) => return Err(error),
            }
        }
        return self.call();
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
                                id: rng().random(),
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
                                id: rng().random(),
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
                                id: rng().random(),
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
                                id: rng().random(),
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

    fn and(&mut self) -> Result<Box<dyn Expression>, String> {
        let mut expr = self.equality()?;
        while self.match_tokens(&vec![TokenType::AND])? {
            let operator = self.previous()?;
            let right = self.equality()?;
            expr = Box::new(LogicalExpression {
                left: expr,
                operator,
                right,
                id: rng().random(),
            })
        }
        return Ok(expr);
    }

    fn or(&mut self) -> Result<Box<dyn Expression>, String> {
        let mut expr = self.and()?;
        while self.match_tokens(&vec![TokenType::OR])? {
            let operator = self.previous()?;
            let right = self.and()?;
            expr = Box::new(LogicalExpression {
                left: expr,
                operator,
                right,
                id: rng().random(),
            })
        }
        return Ok(expr);
    }

    fn assignment(&mut self) -> Result<Box<dyn Expression>, String> {
        let expression = self.or()?;
        if self.match_tokens(&Vec::from([TokenType::EQUAL]))? {
            let equals = self.previous()?;
            let value = self.assignment()?;

            if let Some(var_expr) = expression.as_any().downcast_ref::<VariableExpression>() {
                let name = var_expr.variable.clone();
                return Ok(Box::new(AssignmentExpression {
                    name,
                    value,
                    id: rng().random(),
                }));
            } else if let Some(get_expr) = expression.as_any().downcast_ref::<GetExpression>() {
                return Ok(Box::new(SetExpression {
                    expression: get_expr.expression.clone_box(),
                    name: get_expr.name.clone(),
                    value,
                    id: rng().random(),
                }));
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

    fn while_statement(&mut self) -> Result<WhileStatement, String> {
        self.consume(
            TokenType::LEFT_PAREN,
            String::from("Expect '(' after 'while'."),
        )?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RIGHT_PAREN,
            String::from("Expect ')' after condition."),
        )?;
        let body = self.statement()?;
        return Ok(WhileStatement { condition, body });
    }

    fn for_statement(&mut self) -> Result<Box<dyn Statement>, String> {
        self.consume(
            TokenType::LEFT_PAREN,
            String::from("Expect '(' after 'for'."),
        )?;
        let mut initializer = None;
        if self.match_tokens(&vec![TokenType::SEMICOLON])? {
            initializer = None;
        } else if self.match_tokens(&vec![TokenType::VAR])? {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(Box::new(self.expression_statement()?));
        }
        let mut condition = None;
        if !self.check(TokenType::SEMICOLON)? {
            condition = Some(self.expression()?);
        }
        self.consume(
            TokenType::SEMICOLON,
            String::from("Expect ';' after loop condition."),
        )?;
        let mut increment = None;
        if !self.check(TokenType::RIGHT_PAREN)? {
            increment = Some(self.expression()?);
        }
        self.consume(
            TokenType::RIGHT_PAREN,
            String::from("Expect ')' after for clauses."),
        )?;
        let mut body = self.statement()?;
        match increment {
            Some(inc) => {
                body = Box::new(BlockStatement {
                    statements: vec![body, Box::new(ExpressionStatement { expression: inc })],
                })
            }
            None => {}
        }
        match condition {
            Some(_) => {}
            None => {
                condition = Some(Box::new(Literal {
                    value: LiteralValue::Boolean(true),
                    id: rng().random(),
                }))
            }
        }
        body = Box::new(WhileStatement {
            body,
            condition: condition.unwrap(),
        });
        match initializer {
            Some(init) => {
                body = Box::new(BlockStatement {
                    statements: vec![init, body],
                })
            }
            None => {}
        }
        return Ok(body);
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

    fn block(&mut self) -> Result<BlockStatement, String> {
        let mut statements = Vec::new();
        while !self.check(TokenType::RIGHT_BRACE)? && !self.is_at_end()? {
            statements.push(self.declaration()?);
        }
        self.consume(
            TokenType::RIGHT_BRACE,
            String::from("Expect '}' after block."),
        )?;
        return Ok(BlockStatement { statements });
    }

    fn if_statement(&mut self) -> Result<IfStatement, String> {
        self.consume(
            TokenType::LEFT_PAREN,
            String::from("Expect '(' after 'if'."),
        )?;
        let expression = self.expression()?;
        self.consume(
            TokenType::RIGHT_PAREN,
            String::from("Expect ')' after if condition."),
        )?;
        let then_statement = self.statement()?;
        let mut else_statement = None;
        if self.match_tokens(&vec![TokenType::ELSE])? {
            else_statement = Some(self.statement()?);
        }

        return Ok(IfStatement {
            condition: expression,
            then_statement,
            else_statement,
        });
    }

    fn return_statement(&mut self) -> Result<ReturnStatement, String> {
        let keyword = self.previous()?;
        let mut value: Option<Box<dyn Expression>> = None;
        if !self.check(TokenType::SEMICOLON)? {
            value = Some(self.expression()?);
        }
        self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after return value.".to_string(),
        )?;
        Ok(ReturnStatement { keyword, value })
    }

    fn statement(&mut self) -> Result<Box<dyn Statement>, String> {
        if self.match_tokens(&vec![TokenType::PRINT])? {
            Ok(Box::new(self.print_statement()?))
        } else if self.match_tokens(&vec![TokenType::IF])? {
            return Ok(Box::new(self.if_statement()?));
        } else if self.match_tokens(&vec![TokenType::LEFT_BRACE])? {
            return Ok(Box::new(self.block()?));
        } else if self.match_tokens(&vec![TokenType::WHILE])? {
            return Ok(Box::new(self.while_statement()?));
        } else if self.match_tokens(&vec![TokenType::FOR])? {
            return Ok(self.for_statement()?);
        } else if self.match_tokens(&vec![TokenType::RETURN])? {
            return Ok(Box::new(self.return_statement()?));
        } else {
            Ok(Box::new(self.expression_statement()?))
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
                    id: rng().random(),
                }),
                name,
            })),
        }
    }

    fn class_declaration(&mut self) -> Result<Box<dyn Statement>, String> {
        let class_name = self.consume(TokenType::IDENTIFIER, String::from("Expect class name."))?;
        self.consume(
            TokenType::LEFT_BRACE,
            String::from("Expect '{' before class body."),
        )?;
        let mut methods = Vec::new();
        while !self.is_at_end()? && !self.check(TokenType::RIGHT_BRACE)? {
            methods.push(self.fun_declaration("method".to_string())?);
        }
        self.consume(
            TokenType::RIGHT_BRACE,
            String::from("Expect '}' after class body."),
        )?;
        return Ok(Box::new(ClassStatement {
            name: class_name,
            methods,
        }));
    }

    fn fun_declaration(&mut self, kind: String) -> Result<Box<dyn Statement>, String> {
        let name: Token = self.consume(
            TokenType::IDENTIFIER,
            String::from(format!("Expect {} name.", kind)),
        )?;
        self.consume(
            TokenType::LEFT_PAREN,
            String::from(format!("Expect '(' after {} name.", kind)),
        )?;

        let mut parameters = Vec::new();
        if !self.check(TokenType::RIGHT_PAREN)? {
            parameters.push(self.consume(
                TokenType::IDENTIFIER,
                String::from("Expect parameter name."),
            )?);

            while self.match_tokens(&vec![TokenType::COMMA])? {
                if parameters.len() >= 255 {
                    self.error(
                        self.peek()?,
                        String::from("Can't have more than 255 parameters"),
                    );
                }
                parameters.push(self.consume(
                    TokenType::IDENTIFIER,
                    String::from("Expect parameter name."),
                )?);
            }
        }

        self.consume(
            TokenType::RIGHT_PAREN,
            String::from(format!("Expect ')' after parameters")),
        )?;
        self.consume(
            TokenType::LEFT_BRACE,
            String::from(format!("Expect '{{' before {} body.", kind)),
        )?;

        let statements = self.block()?;

        return Ok(Box::new(FunctionStatement {
            name,
            parameters,
            body: statements.statements,
        }));
    }

    fn declaration(&mut self) -> Result<Box<dyn Statement>, String> {
        if self.match_tokens(&Vec::from([TokenType::VAR]))? {
            return self.var_declaration();
        }
        if self.match_tokens(&vec![TokenType::FUN])? {
            return self.fun_declaration(String::from("function"));
        }
        if self.match_tokens(&vec![TokenType::CLASS])? {
            return self.class_declaration();
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
