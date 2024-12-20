use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    u32,
};

use ast::AST;

mod ast;

/// Complexity: O(1) because we iterate over 32 bits max
fn adder(a: u32, b: u32) -> u32 {
    let mut carry: u32 = a & b;
    let sum: u32 = a ^ b;

    if carry != 0 {
        carry <<= 1;
        return adder(carry, sum);
    }
    sum
}

/// Complexity: O(1) because we iterate over 32 bits max (over arg b)
fn multiplier(a: u32, b: u32) -> u32 {
    let mut res = 0;
    let mut mask = 1;
    let mut shift = 0;

    while mask != 0 {
        if (b & mask) != 0 {
            res = adder(res, a << shift);
        }
        mask <<= 1;
        shift += 1;
    }
    res
}

fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

fn eval_formula(formula: &str) -> bool {
    let tree = AST::try_from(formula.to_string()).unwrap();
    tree.solve()
}

fn print_truth_table(formula: &str) {
    // find the variables in the formula (unique and capital letters only)
    let mut list = formula
        .chars()
        .filter(|c| c.is_ascii_uppercase())
        .collect::<BTreeSet<char>>();

    /*
       A B C
       0 0 0
       1 0 0
       0 1 0
       0 0 1

       0000
       0111
    */

    for c in list.iter() {
        print!("{},", c);
    }
    println!();

    let mut map = list.iter().map(|&c| (c, 0)).collect::<BTreeMap<char, u32>>();

    for i in 0..2u32.pow(list.len() as u32) {
        let mut j = 0;
        for c in list.iter().rev() {
            // print!("{},", (i >> j) & 1);
            map.insert(*c, (i >> j) & 1);
            j += 1;
        }
        println!("{:?}", map);
    }
}

fn main() {
    // println!("ADDER MAIN");
    // println!("{}", adder(5, 5));
    // println!("{}", adder(5, 7));
    // println!("{}", adder(5, 5));
    // println!("{}", adder(5, 0));
    // println!("{}", adder(0, 0));
    // println!("{}", adder(0, 7));
    // println!("{}", adder(u32::MAX, 654));

    // println!("MULTIPLIER MAIN");
    // println!("{}", multiplier(5, 5));
    // println!("{}", multiplier(5, 7));
    // println!("{}", multiplier(5, 5));
    // println!("{}", multiplier(5, 0));
    // println!("{}", multiplier(0, 0));

    // println!("GRAY CODE MAIN");
    // println!("{}", gray_code(0));
    // println!("{}", gray_code(1));
    // println!("{}", gray_code(2));
    // println!("{}", gray_code(3));
    // println!("{}", gray_code(4));
    // println!("{}", gray_code(5));

    let formula = "ABCA";
    print_truth_table(formula);
}

#[cfg(test)]
mod tests {
    use crate::{adder, eval_formula, gray_code, multiplier};

    #[test]
    fn test_adder() {
        assert_eq!(adder(0, 0), 0);
        assert_eq!(adder(1, 0), 1);
        assert_eq!(adder(0, 1), 1);
        assert_eq!(adder(42, 42), 84);
        assert_eq!(adder(100000, 1), 100001);
        assert_eq!(
            adder(3000000000, 3000000000),
            3000000000u32.wrapping_add(3000000000)
        );
    }

    #[test]
    fn test_multiplier() {
        assert_eq!(multiplier(9, 5), 45);
        assert_eq!(multiplier(0, 0), 0);
        assert_eq!(multiplier(1, 0), 0);
        assert_eq!(multiplier(0, 1), 0);
        assert_eq!(multiplier(10, 10), 100);
        assert_eq!(multiplier(100, 100), 10000);
        assert_eq!(
            multiplier(1000000, 1000000),
            1000000u32.wrapping_mul(1000000u32)
        );
    }

    #[test]
    fn test_gray_code() {
        assert_eq!(gray_code(0), 0);
        assert_eq!(gray_code(1), 1);
        assert_eq!(gray_code(2), 3);
        assert_eq!(gray_code(3), 2);
        assert_eq!(gray_code(4), 6);
        assert_eq!(gray_code(5), 7);
        assert_eq!(gray_code(6), 5);
        assert_eq!(gray_code(7), 4);
        assert_eq!(gray_code(8), 12);
    }

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
