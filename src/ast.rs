#[derive(Debug)]
enum Token {
    Operator(Operator),
    Value(Value),
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    BinaryOperator(BinaryOperator),
    Not,
}

#[derive(Debug, Clone, Copy)]
enum BinaryOperator {
    And,
    Or,
    Implies,
    Equivalent,
    Xor,
}

#[derive(Debug)]
enum Value {
    True,
    False,
}

impl TryFrom<char> for Token {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '&' => Ok(Token::Operator(Operator::BinaryOperator(
                BinaryOperator::And,
            ))),
            '|' => Ok(Token::Operator(Operator::BinaryOperator(BinaryOperator::Or))),
            '>' => Ok(Token::Operator(Operator::BinaryOperator(
                BinaryOperator::Implies,
            ))),
            '=' => Ok(Token::Operator(Operator::BinaryOperator(
                BinaryOperator::Equivalent,
            ))),
            '^' => Ok(Token::Operator(Operator::BinaryOperator(
                BinaryOperator::Xor,
            ))),
            '!' => Ok(Token::Operator(Operator::Not)),
            '1' => Ok(Token::Value(Value::True)),
            '0' => Ok(Token::Value(Value::False)),
            _ => Err(format!("Invalid character: {}", value)),
        }
    }
}

#[derive(Debug)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    token: Token,
}

impl Node {
    fn new(token: Token) -> Node {
        Node {
            left: None,
            right: None,
            token: token,
        }
    }

    fn add_left(&mut self, node: Node) {
        self.left = Some(Box::new(node));
    }

    fn add_right(&mut self, node: Node) {
        self.right = Some(Box::new(node));
    }

    fn solve_node(&self) -> bool {
        match self.token {
            Token::Operator(Operator::BinaryOperator(BinaryOperator::And)) => {
                self.left.as_ref().unwrap().solve_node()
                    && self.right.as_ref().unwrap().solve_node()
            }
            Token::Operator(Operator::BinaryOperator(BinaryOperator::Or)) => {
                self.left.as_ref().unwrap().solve_node()
                    || self.right.as_ref().unwrap().solve_node()
            }
            Token::Operator(Operator::BinaryOperator(BinaryOperator::Implies)) => {
                !self.left.as_ref().unwrap().solve_node()
                    || self.right.as_ref().unwrap().solve_node()
            }
            Token::Operator(Operator::BinaryOperator(BinaryOperator::Equivalent)) => {
                self.left.as_ref().unwrap().solve_node()
                    == self.right.as_ref().unwrap().solve_node()
            }
            Token::Operator(Operator::BinaryOperator(BinaryOperator::Xor)) => {
                self.left.as_ref().unwrap().solve_node()
                    != self.right.as_ref().unwrap().solve_node()
            }
            Token::Operator(Operator::Not) => !self.left.as_ref().unwrap().solve_node(),
            Token::Value(Value::True) => true,
            Token::Value(Value::False) => false,
        }
    }
}

#[derive(Debug)]
pub struct AST {
    root: Node,
}

impl AST {
    pub fn solve(&self) -> bool {
        self.root.solve_node()
    }
}

impl TryFrom<String> for AST {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let tokens: Vec<Node> =
            value
                .chars()
                .try_fold(Vec::new(), |mut acc, c| match Token::try_from(c) {
                    Ok(token) => {
                        acc.push(Node::new(token));
                        Ok(acc)
                    }
                    Err(e) => Err(e),
                })?;

        let mut stack: Vec<Node> = vec![];

        for mut token in tokens.into_iter() {
            match token.token {
                Token::Operator(Operator::Not) => {
                    let operand = stack.pop()
                    .ok_or_else(|| String::from("Invalid expression for not operator"))?;
                    token.add_left(operand);
                    stack.push(token);
                }
                Token::Operator(op) => {
                    let right = stack.pop()
                    .ok_or_else(|| format!("Invalid expression for {op:?} operator"))?;
                    let left = stack.pop()
                    .ok_or_else(|| format!("Invalid expression for {op:?} operator"))?;
                    token.add_left(left);
                    token.add_right(right);
                    stack.push(token);
                }
                _ => stack.push(token),
            }
        }
        assert!(stack.len() == 1);
        Ok(AST { root: stack.pop().unwrap() })
    }
}

