//TODO: rename
#[derive(Debug)]
enum Unit {
    Operator(Operator),
    Value(Value),
}

//TODO: check if values have correct names
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

impl TryFrom<char> for Unit {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '&' => Ok(Unit::Operator(Operator::BinaryOperator(
                BinaryOperator::And,
            ))),
            '|' => Ok(Unit::Operator(Operator::BinaryOperator(BinaryOperator::Or))),
            '>' => Ok(Unit::Operator(Operator::BinaryOperator(
                BinaryOperator::Implies,
            ))),
            '=' => Ok(Unit::Operator(Operator::BinaryOperator(
                BinaryOperator::Equivalent,
            ))),
            '^' => Ok(Unit::Operator(Operator::BinaryOperator(
                BinaryOperator::Xor,
            ))),
            '!' => Ok(Unit::Operator(Operator::Not)),
            '1' => Ok(Unit::Value(Value::True)),
            '0' => Ok(Unit::Value(Value::False)),
            _ => Err(format!("Invalid character: {}", value)),
        }
    }
}

#[derive(Debug)]
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    unit: Unit,
}

impl Node {
    fn new(unit: Unit) -> Node {
        Node {
            left: None,
            right: None,
            unit: unit,
        }
    }

    fn add_left(&mut self, node: Node) {
        self.left = Some(Box::new(node));
    }

    fn add_right(&mut self, node: Node) {
        self.right = Some(Box::new(node));
    }

    fn solve_node(&self) -> bool {
        match self.unit {
            Unit::Operator(Operator::BinaryOperator(BinaryOperator::And)) => {
                self.left.as_ref().unwrap().solve_node()
                    && self.right.as_ref().unwrap().solve_node()
            }
            Unit::Operator(Operator::BinaryOperator(BinaryOperator::Or)) => {
                self.left.as_ref().unwrap().solve_node()
                    || self.right.as_ref().unwrap().solve_node()
            }
            Unit::Operator(Operator::BinaryOperator(BinaryOperator::Implies)) => {
                !self.left.as_ref().unwrap().solve_node()
                    || self.right.as_ref().unwrap().solve_node()
            }
            Unit::Operator(Operator::BinaryOperator(BinaryOperator::Equivalent)) => {
                self.left.as_ref().unwrap().solve_node()
                    == self.right.as_ref().unwrap().solve_node()
            }
            Unit::Operator(Operator::BinaryOperator(BinaryOperator::Xor)) => {
                self.left.as_ref().unwrap().solve_node()
                    != self.right.as_ref().unwrap().solve_node()
            }
            Unit::Operator(Operator::Not) => !self.left.as_ref().unwrap().solve_node(),
            Unit::Value(Value::True) => true,
            Unit::Value(Value::False) => false,
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
                .try_fold(Vec::new(), |mut acc, c| match Unit::try_from(c) {
                    Ok(unit) => {
                        acc.push(Node::new(unit));
                        Ok(acc)
                    }
                    Err(e) => Err(e),
                })?;
        println!("{:?}", tokens);

        let mut stack: Vec<Node> = vec![];

        for mut token in tokens.into_iter() {
            match token.unit {
                Unit::Operator(Operator::Not) => {
                    let operand = stack.pop()
                    .ok_or_else(|| String::from("Invalid expression for not operator"))?;
                    token.add_left(operand);
                    stack.push(token);
                }
                Unit::Operator(op) => {
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

