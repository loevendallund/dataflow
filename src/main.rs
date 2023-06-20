#![feature(box_into_inner)]

use crate::convExprToOcc::convert;

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod occParser;
pub mod exprParser;
pub mod convExprToOcc;
pub mod evaluator;

#[cfg(test)]
mod tests;

fn main()
{

    //let ReadFile = "(100)".to_string();
    //let ReadFile = "((func x.(x)) (10))".to_string();
    //let ReadFile = "(PLUS (5) (5))".to_string();
    //let ReadFile = "(letrec x (func y.(y)) ((x) (5)))".to_string();
    //let ReadFile = "(let x (ref (100)) (let z ((x):=(5)) (!(x))))".to_string();
    let ReadFile = "((func z.(letrec fac (func y.(case (y) (0,_)((1),(TIMES (y) ((fac) (MINUS (y) (1))))))) ((fac) (z)))) (4))".to_string(); // Factorial


    //let occ = occParser::Parse_Expr(ReadFile);
    let expr: exprParser::Expr = exprParser::parser(ReadFile);
    println!("{:#?}",expr);
    let occ = convert(expr);
    println!("{:#?}",occ);

    let val = evaluator::intepret(occ);
    println!("{:#?}",val);

}

