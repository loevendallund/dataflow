#![feature(box_into_inner)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod occParser;
pub mod exprParser;
pub mod convExprToOcc;
pub mod evaluator;
pub mod typechecker;

#[cfg(test)]
mod tests;

use std::collections::HashMap;

use crate::convExprToOcc::convert;
use crate::evaluator::Val;
use crate::evaluator::SOcc;
use crate::typechecker::SemOcc;

fn main()
{
    //let ReadFile = "(true)".to_string();
    let ReadFile = "((func x.(x)) (10))".to_string();
    //let ReadFile = "(PLUS (10) (20))".to_string();
    //let ReadFile = "(let x (let z (1) (z)) (x))".to_string();
    //let ReadFile = "(let x (let z (1) (z)) (PLUS (x) (20)))".to_string();
    //let ReadFile = "(let y (1) (case (y) (1,x)((10),(20))))".to_string();
    //let ReadFile = "(case (LESS (1) (2)) (true,false)((10),(20)))".to_string();
    //let ReadFile = "((func x.(x)) (let y (let z (1) (z)) (y)))".to_string();
    //let ReadFile = "(let x (10) (x))".to_string();
    //let ReadFile = "(PLUS (5) (5))".to_string();
    //let ReadFile = "(letrec x (func y.(y)) ((x) (5)))".to_string();
    //let ReadFile = "(let x (ref (100)) (!(x)))".to_string();
    //let ReadFile = "(let x (ref (100)) (let z ((x):=(5)) (!(x))))".to_string();
    //let ReadFile = "(let x (ref (100)) (let z (ref (x)) (!(z))))".to_string();
    //let ReadFile = "(let x (ref (100)) (let z (let q (ref (x)) ((x):=(10))) (!(x))))".to_string();
    //let ReadFile = "(let x (ref (let y (1) (y))) (let z ((x):=(5)) (!(x))))".to_string();
    //let ReadFile = "(let x (ref (10)) ((x):=(10)))".to_string();
    //let ReadFile = "(!(let x (ref (10)) (case (1) (1,2)((10),(let z ((x):=(2)) (x))))))".to_string();
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
    //(val, w, gbind, (L,V)) = evaluator::intepret(occ.clone());
    //println!("{:#?}",val);
    //println!("dependency function: \n{:#?}",w);
    //println!("greatest bindings: \n{:#?}",gbind);
    //println!("dependencies: \n({:#?}, {:#?})",L, V);
    
    let mut Gamma: HashMap<SemOcc, typechecker::Type> = HashMap::new();
    let mut Pi: HashMap<usize, usize> = HashMap::new();
    Pi.insert(1,2);
    Pi.insert(2,3);
    Pi.insert(3,4);
    Pi.insert(5,6);
    Pi.insert(6,7);
    Pi.insert(7,8);
    Pi.insert(8,9);
    Pi.insert(9,10);
    Pi.insert(4,10);
    Pi.insert(10,11);
    Pi.insert(11,12);

    let mut Pi1: HashMap<usize, usize> = HashMap::new();
    Pi1.insert(1,2);
    Pi1.insert(2,3);
    Pi1.insert(3,4);
    Pi1.insert(4,10);
    Pi1.insert(10,11);
    Pi1.insert(11,12);
    let mut Pi2: HashMap<usize, usize> = HashMap::new();
    Pi2.insert(1,2);
    Pi2.insert(2,3);
    Pi2.insert(2,7);
    Pi2.insert(5,6);
    Pi2.insert(6,7);
    Pi2.insert(7,8);
    Pi2.insert(8,9);
    Pi2.insert(9,10);
    Pi2.insert(10,11);
    Pi2.insert(11,12);
    let v: Vec<HashMap<usize,usize>>= vec![Pi1, Pi2];
    let t: typechecker::Type;
    (t, Gamma) = typechecker::TCheck(Gamma, Pi, occ.clone(), v, None);
    println!("{:#?}",t);
    println!("Gamma: {:#?}",Gamma);

}

