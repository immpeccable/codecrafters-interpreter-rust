use crate::enums::LiteralValue::LiteralValue;
use crate::enums::TokenType::TokenType;
use crate::implementation::BinaryExpression::BinaryExpression;
use crate::implementation::Grouping::Grouping;
use crate::implementation::Literal::Literal;
use crate::implementation::Token::Token;
use crate::implementation::UnaryExpression::UnaryExpression;
use crate::traits::Expression::Expression;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    fn match_tokens(&mut self, token_types: &Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(*token_type) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(&self, token_type: TokenType) -> bool {
        match self.peek() {
            Some(token) => token.token_type == token_type,
            None => false,
        }
    }

    fn previous(&self) -> Option<Token> {
        return self.tokens.get(self.current - 1).cloned();
    }

    fn peek(&self) -> Option<Token> {
        return self.tokens.get(self.current).cloned();
    }

    fn is_at_end(&self) -> bool {
        match self.peek() {
            Some(token) => token.token_type == TokenType::EOF,
            None => false,
        }
    }

    fn advance(&mut self) -> Option<Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn consume(&mut self, token_type: TokenType, message: &'static str) -> Option<Token> {
        if self.check(token_type) {
            return self.advance();
        }
        panic!("{:?} {}", self.peek().unwrap(), message);
    }

    fn primary(&mut self) -> Box<dyn Expression> {
        if self.match_tokens(&[TokenType::FALSE].to_vec()) {
            return Box::new(Literal {
                value: LiteralValue::False,
            });
        } else if self.match_tokens(&[TokenType::TRUE].to_vec()) {
            return Box::new(Literal {
                value: LiteralValue::True,
            });
        } else if self.match_tokens(&[TokenType::NIL].to_vec()) {
            return Box::new(Literal {
                value: LiteralValue::Nil,
            });
        } else if self.match_tokens(&[TokenType::NUMBER, TokenType::STRING].to_vec()) {
            match self.previous() {
                Some(token) => {
                    if token.token_type == TokenType::STRING {
                        return Box::new(Literal {
                            value: LiteralValue::String(token.token_value),
                        });
                    } else if token.token_type == TokenType::NUMBER {
                        return Box::new(Literal {
                            value: LiteralValue::Number(token.token_value),
                        });
                    }
                }
                None => {}
            }
        } else if self.match_tokens(&[TokenType::LEFT_PAREN].to_vec()) {
            let expression = self.expression();
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.");
            return Box::new(Grouping { expression });
        }
        panic!("WTF is going on");
    }

    fn unary(&mut self) -> Box<dyn Expression> {
        if self.match_tokens(&[TokenType::BANG, TokenType::MINUS].to_vec()) {
            let operator = self.previous();
            let right = self.unary();
            return Box::new(UnaryExpression {
                operator: operator.unwrap(),
                expression: right,
            });
        }
        return self.primary();
    }

    fn factor(&mut self) -> Box<dyn Expression> {
        let mut expression = self.unary();
        while self.match_tokens(&[TokenType::SLASH, TokenType::STAR].to_vec()) {
            let operator = self.previous();
            let right = self.unary();
            expression = Box::new(BinaryExpression {
                left: expression,
                operator: operator.unwrap(),
                right,
            })
        }
        return expression;
    }

    fn term(&mut self) -> Box<dyn Expression> {
        let mut expression = self.factor();
        while self.match_tokens(&[TokenType::MINUS, TokenType::PLUS].to_vec()) {
            let operator = self.previous();
            let right = self.factor();
            expression = Box::new(BinaryExpression {
                left: expression,
                operator: operator.unwrap(),
                right,
            })
        }
        return expression;
    }

    fn comparison(&mut self) -> Box<dyn Expression> {
        let mut expression = self.term();
        while self.match_tokens(
            &[
                TokenType::LESS,
                TokenType::LESS_EQUAL,
                TokenType::GREATER,
                TokenType::GREATER_EQUAL,
            ]
            .to_vec(),
        ) {
            let operator = self.previous();
            let right = self.term();
            expression = Box::new(BinaryExpression {
                left: expression,
                operator: operator.unwrap(),
                right,
            })
        }
        return expression;
    }

    fn equality(&mut self) -> Box<dyn Expression> {
        let mut expression = self.comparison();
        while self.match_tokens(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL].to_vec()) {
            let operator = self.previous();
            let right = self.comparison();
            expression = Box::new(BinaryExpression {
                left: expression,
                operator: operator.unwrap(),
                right,
            })
        }

        return expression;
    }

    pub fn expression(&mut self) -> Box<dyn Expression> {
        return self.equality();
    }
}
