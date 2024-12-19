use std::u32;

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



fn main() {
    let expr = "110|&";
    let tree = AST::try_from(expr.to_string()).unwrap();

    println!("{:?}", tree);

    //------------------------------
    println!("ADDER MAIN");
    println!("{}", adder(5, 5));
    println!("{}", adder(5, 7));
    println!("{}", adder(5, 5));
    println!("{}", adder(5, 0));
    println!("{}", adder(0, 0));
    println!("{}", adder(0, 7));
    println!("{}", adder(u32::MAX, 654));

    println!("MULTIPLIER MAIN");
    println!("{}", multiplier(5, 5));
    println!("{}", multiplier(5, 7));
    println!("{}", multiplier(5, 5));
    println!("{}", multiplier(5, 0));
    println!("{}", multiplier(0, 0));

    println!("GRAY CODE MAIN");
    println!("{}", gray_code(0));
    println!("{}", gray_code(1));
    println!("{}", gray_code(2));
    println!("{}", gray_code(3));
    println!("{}", gray_code(4));
    println!("{}", gray_code(5));
}


#[cfg(test)]
mod tests {
    use crate::{adder, gray_code, multiplier};

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
}