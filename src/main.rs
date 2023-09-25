#![feature(box_into_inner)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod occParser;
pub mod exprParser;
pub mod convExprToOcc;
pub mod evaluator;
pub mod typechecker;
pub mod approxOrder;
pub mod tc;

#[cfg(test)]
#[path = "tests/tests.rs"]
pub mod tests;

use std::collections::HashMap;

use crate::approxOrder::approx;
use crate::convExprToOcc::convert;
use crate::evaluator::Val;
use crate::evaluator::SOcc;
use crate::tc::TypeChecker;
use crate::typechecker::SemOcc;

fn main()
{
    //let ReadFile = "(true)".to_string();
    //let ReadFile = "((func x.(x)) (10))".to_string();
    //let ReadFile = "(PLUS (10) (20))".to_string();
    //let ReadFile = "(let x (1) (x))".to_string();
    //let ReadFile = "(let x (let z (1) (z)) (x))".to_string();
    //let ReadFile = "(let x (let z (1) (z)) (PLUS (x) (20)))".to_string();
    //let ReadFile = "(let y (1) (case (y) (1,x)((10),(20))))".to_string();
    //let ReadFile = "(case (LESS (1) (2)) (true,false)((10),(20)))".to_string();
    //let ReadFile = "((func x.(x)) (let y (let z (1) (z)) (y)))".to_string();
    //let ReadFile = "(let x (10) (x))".to_string();
    //let ReadFile = "(PLUS (5) (5))".to_string();
    //let ReadFile = "(letrec x (func y.(y)) ((x) (5)))".to_string();
    let ReadFile = "(let x (ref (100)) (!(x)))".to_string();
    //let ReadFile = "(let x (ref (100)) (let z ((x):=(5)) (!(x))))".to_string();
    //let ReadFile = "(let x (ref (100)) (let z (ref (x)) (!(z))))".to_string();
    //let ReadFile = "(let x (ref (100)) (let z (let q (ref (x)) ((x):=(10))) (!(x))))".to_string();
    //let ReadFile = "(let x (ref (let y (1) (y))) (let z ((x):=(5)) (!(x))))".to_string();
    //let ReadFile = "(let x (ref (10)) ((x):=(10)))".to_string();
    //let ReadFile = "(!(let x (ref (10)) (case (1) (1,2)((10),(let z ((x):=(2)) (x))))))".to_string();
    //let ReadFile = "((func z.(letrec fac (func y.(case (y) (0,_)((1),(TIMES (y) ((fac) (MINUS (y) (1))))))) ((fac) (z)))) (4))".to_string(); // Factorial
    //let ReadFile = "(let x (ref (3)) (let y (let z (5) ((x):=(z)))(!(x))))".to_string();


    //let occ = occParser::Parse_Expr(ReadFile);
    let expr: exprParser::Expr = exprParser::parser(ReadFile);
    //println!("{:#?}",expr);
    let occ = convert(expr);
    //println!("{:#?}",occ);

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
    //let mut Pi: HashMap<usize, usize> = HashMap::new();
    //Pi.insert(3,1);
    //Pi.insert(2,3);
    //Pi.insert(1,4);
   /* Pi.insert(5,6);
    Pi.insert(6,7);
    Pi.insert(7,8);
    Pi.insert(8,9);
    Pi.insert(9,10);
    Pi.insert(4,10);
    Pi.insert(10,11);
    Pi.insert(11,12);*/

    /*let mut Pi1: HashMap<usize, usize> = HashMap::new();
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
    let v: Vec<HashMap<usize,usize>>= vec![Pi1, Pi2];*/
    let Pi = approx(occ.clone());
    //let t: typechecker::Type;
    //(t, Gamma) = typechecker::TCheck(Gamma, Pi.clone(), occ.clone(), Vec::new(), None);
    //println!("Old: {:#?}",t);

    let mut gamma1: HashMap<tc::SemOcc, tc::Type> = HashMap::new();

    let mut _Pi: tc::Pi = tc::Pi { p: Vec::new() };
    //_Pi.construct_from_hash(Pi);
    let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma: gamma1, pi: Pi, occ, assumption: Vec::new() };
    
    //let tc_t: tc::Type = _tc.type_check();

    //println!("New: {:#?}",tc_t);
    //println!("{:#?}",_tc.gamma);


	/*let mut p: Vec<(usize, usize)> = Vec::new();
	p.push((0,1));
	p.push((1,2));
	p.push((2,3));
	p.push((3,5));
	p.push((2,4));
	p.push((4,6));
	p.push((4,7));
	p.push((6,5));
	p.push((7,5));

	let pi: tc::Pi = tc::Pi { p };
	let res: Vec<tc::Pi> = pi.clone().get_all_pi(5);
   println!("res: {:#?}",res);*/
    /*let mut p: Vec<(usize, usize)> = Vec::new();
	p.push((0,1));
	p.push((1,2));
	p.push((2,3));

	let pi: tc::Pi = tc::Pi { p };
   println!("res: {:#?}",pi.clone());
   println!("res: {:#?}",pi.clone().construct_total());*/
}
