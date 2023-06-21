#![feature(box_into_inner)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod occParser;
pub mod exprParser;
pub mod convExprToOcc;
pub mod evaluator;

#[cfg(test)]
mod tests;

use std::collections::HashMap;

use crate::convExprToOcc::convert;
use crate::evaluator::Val;
use crate::evaluator::SOcc;

fn main()
{
    //let ReadFile = "(true)".to_string();
    //let ReadFile = "(case (LESS (1) (2)) (true,false)((10),(20)))".to_string();
    //let ReadFile = "((func x.(x)) (let y (let z (1) (z)) (y)))".to_string();
    //let ReadFile = "(let x (10) (x))".to_string();
    //let ReadFile = "(PLUS (5) (5))".to_string();
    //let ReadFile = "(letrec x (func y.(y)) ((x) (5)))".to_string();
    //let ReadFile = "(let x (ref (100)) (!(x)))".to_string();
    //let ReadFile = "(let x (ref (100)) (let z ((x):=(5)) (!(x))))".to_string();
    //let ReadFile = "(let x (ref (100)) (let z (ref (x)) (!(z))))".to_string();
    //let ReadFile = "(let x (ref (100)) (let z (let q (ref (x)) ((x):=(10))) (!(x))))".to_string();
    let ReadFile = "(let x (ref (let y (1) (y))) (let z ((x):=(5)) (!(x))))".to_string();
    //let ReadFile = "(!(ref (10)))".to_string();
    //let ReadFile = "((func z.(letrec fac (func y.(case (y) (0,_)((1),(TIMES (y) ((fac) (MINUS (y) (1))))))) ((fac) (z)))) (4))".to_string(); // Factorial


    //let occ = occParser::Parse_Expr(ReadFile);
    let expr: exprParser::Expr = exprParser::parser(ReadFile);
    //println!("{:#?}",expr);
    let occ = convert(expr);
    println!("{:#?}",occ);

    let mut w: HashMap<SOcc, (Vec<SOcc>, Vec<SOcc>)> = HashMap::new();
    let mut gbind: HashMap<String, usize> = HashMap::new();
    let val: Val;
    let mut L: Vec<SOcc>;
    let mut V: Vec<SOcc>;
    (val, w, gbind, (L,V)) = evaluator::intepret(occ);
    println!("{:#?}",val);
    println!("dependency function: \n{:#?}",w);
    println!("greatest bindings: \n{:#?}",gbind);
    println!("dependencies: \n({:#?}, {:#?})",L, V);

}

