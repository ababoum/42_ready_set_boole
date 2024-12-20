use std::{
    collections::{BTreeMap, BTreeSet},
    u32,
};

use ast::AST;

mod nnf;
mod ast;

fn eval_formula(formula: &str) -> bool {
    let tree = AST::try_from(formula.to_string()).unwrap();
    tree.solve()
}

//TODO: Anton delete horrible for loops please
//TODO: Anton delete horrible .to_string() please
fn print_truth_table(formula: &str) {
    // find the variables in the formula (unique and capital letters only)
    let list = formula
        .chars()
        .filter(|c| c.is_ascii_uppercase())
        .collect::<BTreeSet<char>>();

    for c in list.iter() {
        print!("| {} ", c);
    }
    println!("| = |");

    for _ in list.iter() {
        print!("|---");
    }
    println!("|---|");

    let mut map = list
        .iter()
        .map(|&c| (c, 0))
        .collect::<BTreeMap<char, u32>>();

    for i in 0..2u32.pow(list.len() as u32) {
        let mut j = 0;
        let mut boolean_formula = formula.to_string();
        for c in list.iter().rev() {
            map.insert(*c, (i >> j) & 1);
            j += 1;
        }
        for (k, v) in map.iter() {
            boolean_formula = boolean_formula.replace(&k.to_string(), &v.to_string());
        }
        map.insert('=', eval_formula(&boolean_formula) as u32);
        for c in list.iter() {
            print!("| {} ", map[c]);
        }
        println!("| {} |", map[&'=']);
    }
}

fn main() {
    let formula = "AB&C|";
    print_truth_table(formula);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_eval_formula() {
        assert_eq!(eval_formula("1"), true);
        assert_eq!(eval_formula("0"), false);
        assert_eq!(eval_formula("10&"), false);
        assert_eq!(eval_formula("10|"), true);
        assert_eq!(eval_formula("11>"), true);
        assert_eq!(eval_formula("00>"), true);
        assert_eq!(eval_formula("10>"), false);
        assert_eq!(eval_formula("01>"), true);
        assert_eq!(eval_formula("10="), false);
        assert_eq!(eval_formula("1011||="), true);
    }
}
