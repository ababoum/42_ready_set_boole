use crate::ast::AST;

/// The result must only contain variables and the following symbols: !, & and | (even if the input contains other operations). Negation operator should be followed by a variable.
#[allow(dead_code)]
fn negation_normal_form(formula: &str) -> String
{
    let ast = AST::try_from(formula.to_string()).unwrap();
    todo!()
}