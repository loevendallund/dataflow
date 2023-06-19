#![feature(box_into_inner)]

use crate::convExprToOcc::convert;

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod occParser;
pub mod exprParser;
pub mod convExprToOcc;
pub mod evaluator;

fn main()
{
    //let ReadFile = fs::read_to_string("test.df").expect("cannot read file");
    //let ReadFile = "(100^1)".to_string();
    //let ReadFile = "(x^1)".to_string();
    //let ReadFile = "((func x.(x^1)^2) (5^5)^6)".to_string();
    //let ReadFile = "(let ex (10^1) (ex^2)^3)".to_string();
    //let ReadFile = "(let x (let y (1^1) (2^2)^3) (x^4)^5)".to_string();
    //let ReadFile = "(let x (func y.(x^1)^2) ((x^3) ((x^4) (5^5)^6)^7)^8)".to_string();
    //let ReadFile = "(let x (func x.(PLUS (x^1) (1^2)^3)^4) ((x^5) ((x^6) (10^7)^8)^9)^10)".to_string();
    //let ReadFile = "(PLUS (1^1) (2^2)^3)".to_string();
    //let ReadFile = "(MINUS (5^1) (3^2)^3)".to_string();
    //let ReadFile = "(TIMES (5^1) (2^2)^3)".to_string();
    //let ReadFile = "(case (3^1) (1,2,3)((10^2),(20^3),(30^4))^5)".to_string();
    //let ReadFile = "(let x (ref (1^1)^2) (!(x^2)^3)^4)".to_string();
    //let ReadFile = "(let x (ref (1^1)^2) ((x^2):=(10^3)^4)^5)".to_string();
    
    //let ReadFile = "(letrec x (func y.(x^1)^2) (x^2)^3)".to_string();
    //let ReadFile = "(letrec x (func y.(PLUS (y^1) (1^2)^3)^4) ((x^3) (10^4)^5)^6)".to_string();
    //let ReadFile = "(letrec p (func y.(case (y^1) (p)((10^2))^5)^4) ((p^3) (10^4)^5)^6)".to_string();
    //let ReadFile = "((func x.(letrec x (func y.(case (y^1) (x)((10^2))^5)^4) ((x^3) (10^4)^5)^6)^2) (1^1)^1)".to_string();
    //let ReadFile = "(letrec p (func y.(case (y^1) (0,_)((10^2),(20^2))^5)^4) ((p^3) (0^4)^5)^6)".to_string();
    //let ReadFile = "(func y.(letrec p (func y.(case (y^1) (0,_)((10^2),(20^2))^5)^4) ((p^3) (0^4)^5)^6)^8)".to_string();
    //let ReadFile = "((func z.(letrec p (func y.(case (y^1) (0,_)((1^2),(TIMES (y^1) (2^2)^3))^5)^4) ((p^3) (5^4)^5)^6)^8) (1^10)^11)".to_string();


    //let ReadFile = "(!(ref (100^1)^2)^3)".to_string();
    //let ReadFile = "(let x (ref (100^1)^2) (!(x^3)^4)^5)".to_string();
    //let ReadFile = "(let x (ref (100^1)^2) (let z ((x^3):=(5^4)^5) (!(x^7)^8)^9)^10)".to_string();
    
    //let ReadFile = "((func z.(letrec fac (func y.(case (y^1) (0,_)((1^2),(TIMES (y^1) ((fac^1) (MINUS (y^1) (1^2)^3)^3)^3))^5)^4) ((fac^3) (z^4)^5)^6)^8) (4^10)^11)".to_string(); // Factorial

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

