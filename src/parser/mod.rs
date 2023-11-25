use crate::tokenizer::{Token, TokenType};

// statement
#[derive(Clone, Debug)]
pub enum Statement {
    BlockStatement(BlockStatement),
    VariableDeclareStatement(VariableDeclareStatement),
}

#[derive(Clone, Debug)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug)]
enum VariableDeclareKind {
    Let,
    Const,
}

#[derive(Clone, Debug)]
pub struct VariableDeclareStatement {
    pub declare_kind: VariableDeclareKind,
    pub name: String,
    pub declared_type: Option<Type>,
    pub init: Expression,
}

// type
#[derive(Clone, Debug)]
pub enum Type {}

// expression
#[derive(Clone, Debug)]
pub enum Expression {
    NumberLiteral(NumberLiteral),
}

#[derive(Clone, Debug)]
pub struct NumberLiteral {}

// parser
pub fn parser<'a>(mut tokens: &'a [Token<'a>]) -> Option<BlockStatement> {
    let mut statements = Vec::new();
    loop {
        if tokens.is_empty() || tokens.first().unwrap().kind == TokenType::Eof {
            break;
        }
        let stmt = parser_statement(tokens);
        println!("{:?}", stmt);
        match stmt {
            (None, _) => (),
            (Some(stmt), _) => statements.push(stmt),
        };
        tokens = stmt.1;
    }
    Some(BlockStatement { statements })
}

fn parser_statement<'a>(tokens: &'a [Token<'a>]) -> (Option<Statement>, &'a [Token<'a>]) {
    match tokens.first().unwrap().kind {
        TokenType::Error => todo!(),
        TokenType::Eof => todo!(),
        TokenType::Comment => todo!(),
        TokenType::LeftParen => todo!(),
        TokenType::RightParen => todo!(),
        TokenType::LeftBrace => todo!(),
        TokenType::RightBrace => todo!(),
        TokenType::Comma => todo!(),
        TokenType::Dot => todo!(),
        TokenType::Minus => todo!(),
        TokenType::Plus => todo!(),
        TokenType::Colon => todo!(),
        TokenType::Semicolon => todo!(),
        TokenType::Slash => todo!(),
        TokenType::Star => todo!(),
        TokenType::Bang => todo!(),
        TokenType::Equal => todo!(),
        TokenType::EqualEqual => todo!(),
        TokenType::Greater => todo!(),
        TokenType::GreaterEqual => todo!(),
        TokenType::Less => todo!(),
        TokenType::LessEqual => todo!(),
        TokenType::Identifier => todo!(),
        TokenType::String => todo!(),
        TokenType::Number => todo!(),
        TokenType::If => todo!(),
        TokenType::Else => todo!(),
        TokenType::While => todo!(),
        TokenType::For => todo!(),
        TokenType::False => todo!(),
        TokenType::True => todo!(),
        TokenType::Const | TokenType::Let => match parser_variable_declare_statement(tokens) {
            (Some(stmt), tokens) => (Some(Statement::VariableDeclareStatement(stmt)), tokens),
            (None, tokens) => (None, tokens),
        },
        TokenType::Function => todo!(),
        TokenType::Class => todo!(),
        TokenType::Null => todo!(),
        TokenType::Return => todo!(),
        TokenType::Break => todo!(),
        TokenType::Continue => todo!(),
    }
}

fn parser_variable_declare_statement<'a>(
    tokens: &'a [Token<'a>],
) -> (Option<VariableDeclareStatement>, &'a [Token<'a>]) {
    let (first, tokens) = tokens.split_first().unwrap();
    let declare_kind = match first.kind {
        TokenType::Const => VariableDeclareKind::Const,
        TokenType::Let => VariableDeclareKind::Let,
        _ => panic!(""),
    };
    let (first, mut tokens) = tokens.split_first().unwrap();
    let identifier: &str = match first.kind {
        TokenType::Identifier => first.lexeme,
        _ => {
            println!("Expect Identifier");
            return (None, skip_until_semicolon(tokens));
        }
    };

    let declared_type = match tokens.first().unwrap().kind {
        TokenType::Colon => {
            let (declared_type, new_tokens) = parser_type(&tokens[1..]);
            tokens = new_tokens;
            declared_type
        }
        TokenType::Eof => todo!(),
        _ => None,
    };

    let (first, mut tokens) = tokens.split_first().unwrap();
    let expr = match first.kind {
        TokenType::Equal => {
            let (expr, new_tokens) = parser_expression(tokens);
            tokens = new_tokens;
            match expr {
                None => return (None, skip_until_semicolon(tokens)),
                Some(expr) => expr,
            }
        }
        _ => {
            println!("Expect initialization");
            return (None, skip_until_semicolon(tokens));
        }
    };
    let (first, tokens) = tokens.split_first().unwrap();
    match first.kind {
        TokenType::Semicolon => (),
        _ => {
            println!("Expect ';'");
            return (None, tokens);
        }
    }

    (
        Some(VariableDeclareStatement {
            declare_kind,
            name: identifier.to_string(),
            declared_type,
            init: expr,
        }),
        tokens,
    )
}

fn parser_expression<'a>(tokens: &'a [Token<'a>]) -> (Option<Expression>, &'a [Token<'a>]) {
    match tokens.first().unwrap().kind {
        TokenType::Number => match parser_number_literal(tokens) {
            (Some(expr), tokens) => (Some(Expression::NumberLiteral(expr)), tokens),
            (None, tokens) => (None, tokens),
        },
        _ => todo!(),
    }
}
fn parser_number_literal<'a>(tokens: &'a [Token<'a>]) -> (Option<NumberLiteral>, &'a [Token<'a>]) {
    match tokens.first().unwrap() {
        token if token.kind == TokenType::Number => (Some(token.lexeme), &tokens[1..]),
        _ => panic!(),
    }
}

fn parser_type<'a>(tokens: &'a [Token<'a>]) -> (Option<Type>, &'a [Token<'a>]) {
    todo!()
}

fn skip_until_semicolon<'a>(tokens: &'a [Token<'a>]) -> &'a [Token<'a>] {
    for i in 0..tokens.len() {
        match tokens.get(i).unwrap().kind {
            TokenType::Eof => {
                return &[];
            }
            TokenType::Semicolon => {
                return tokens.split_at(i + 1).1;
            }
            _ => {}
        };
    }
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::Tokenizer;

    #[test]
    fn parser_test() {
        let code = "let a = 10;";
        let tokens = Tokenizer::new(&code).tokenizer();
        let stmt = parser(&tokens);
        assert!(stmt.is_some());
    }
}
