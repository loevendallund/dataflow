#![feature(box_into_inner)]

use std::fs;
use std::io;
use std::env;

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
use crate::evaluator::intepret;
use crate::tc::TypeChecker;
use crate::typechecker::SemOcc;

fn main()
{
    let args: Vec<String> = env::args().collect();
    let File = fs::read_to_string(&args[1]);
    let ReadFile = match File 
    {
        Ok(file) => file.replace("\n", "").replace("\t", ""),
        Err(error) => panic!("Encountered an error while loading the file: {:?}", error)
    };
    
    let mut evaluate: bool = false;
    if 2 < args.len()
    {
        evaluate = &args[2] == "true";
    }

    let mut debug: bool = false;
    if 3 < args.len()
    {
        debug = &args[3] == "true";
    }

    let expr: exprParser::Expr = exprParser::parser(ReadFile);
    if debug {dbg!(&expr);}

    let occ = convert(expr);
    if debug {dbg!(&occ);}

    if evaluate
    {
        let res = intepret(occ.clone());
        println!("intepreted res: {:#?}", res);
    }

    let mut Gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    let Pi = approx(occ.clone());

    let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma: Gamma, pi: Pi.clone(), occ: occ.clone(), assumption: Vec::new() };
    
    let tc_t: tc::Type = _tc.type_check();

    if debug
    {
        println!("Approximated order: {:#?}", &Pi);
        println!("Gamma: {:#?}", &_tc.gamma);
    }

    println!("Type checked result: {:#?}",tc_t);
}
