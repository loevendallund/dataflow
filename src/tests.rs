use std::collections::HashMap;

use crate::occParser;
use crate::exprParser;
use crate::convExprToOcc;
use crate::evaluator;
use crate::tc;
use crate::tc::SemOcc;

fn factorial(num: usize) -> usize
{
    match num
    {
        0 => { return 1; }
        _ => { return num * (factorial(num - 1)); }
    }
}

fn empty_base() -> tc::Type { tc::Type::Base { delta: Vec::new(), kappa: Vec::new() } }

/*#[test]
fn fac()
{
    // Factorial occurrence, which uses functions, applications, recursive bindings, and functional
    // applications

    let fac_number: usize = 4;
    let res: usize = factorial(fac_number);

    let string = "((func z.(letrec fac (func y.(case (y^1) (0,_)((1^2),(TIMES (y^3) ((fac^4) (MINUS (y^5) (1^6)^7)^8)^9))^10)^11) ((fac^12) (z^13)^14)^15)^16) (".to_string() + &fac_number.to_string() + "^17)^18)"; // Factorial
    let occ = occParser::Parse_Expr(string);
    let val: evaluator::Val = evaluator::intepret(occ);
    
    assert!(match val { evaluator::Val::Const(x) => { x == res } _ => {false} });

    let string = "((func z.(letrec fac (func y.(case (y) (0,_)((1),(TIMES (y) ((fac) (MINUS (y) (1))))))) ((fac) (z)))) (".to_string() + &fac_number.to_string() + "))"; // Factorial
    let expression = exprParser::parser(string);
    let occConv = convExprToOcc::convert(expression);
    let valConv: evaluator::Val = evaluator::intepret(occConv);

    let res2: usize;
    match val { evaluator::Val::Const(x) => { res2 = x; } _ => { unreachable!() } };
    assert!(match valConv { evaluator::Val::Const(x) => { x == res && x == res2 } _ => {false} });
}*/

#[test]
fn const_occ()
{
	let occurrence = "(1)".to_string();

	let expr: exprParser::Expr = exprParser::parser(occurrence);
	let occ = convExprToOcc::convert(expr);
	
	let gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
	let pi: tc::Pi = tc::Pi { p: Vec::new() };
	let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption: Vec::new()};
	let tc_t: tc::Type = _tc.type_check();


	let base = tc::Type::Base { delta: Vec::new() , kappa: Vec::new() };
	
	assert!(tc_t == base);
}

#[test]
fn const_err_occ()
{
	let occurrence = "(1)".to_string();

	let expr: exprParser::Expr = exprParser::parser(occurrence);
	let occ = convExprToOcc::convert(expr);
	
	let gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
	let pi: tc::Pi = tc::Pi { p: Vec::new() };
	let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption: Vec::new()};
	let tc_t: tc::Type = _tc.type_check();

	let mut delta: Vec<SemOcc> = Vec::new();
	delta.push(tc::SemOcc {ident: "x".to_string(), label: 1});
	let base = tc::Type::Base { delta , kappa: Vec::new() };
	
	assert!(tc_t != base);
}

#[test]
fn var_occ()
{
	let occurrence = "(x)".to_string();

	let expr: exprParser::Expr = exprParser::parser(occurrence);
	let occ = convExprToOcc::convert(expr);
	
	let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
	gamma.insert(tc::SemOcc {ident: "x".to_string(), label: 2}, empty_base());
	let pi: tc::Pi = tc::Pi { p: Vec::new() };
	let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption: Vec::new()};
	let tc_t: tc::Type = _tc.type_check();


	let mut delta: Vec<SemOcc> = Vec::new();
	delta.push(tc::SemOcc {ident: "x".to_string(), label: 1});
	let base = tc::Type::Base { delta , kappa: Vec::new() };
	
	assert!(tc_t == base);
}

#[test]
fn var_err_occ()
{
	let occurrence = "(x)".to_string();

	let expr: exprParser::Expr = exprParser::parser(occurrence);
	let occ = convExprToOcc::convert(expr);
	
	let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
	gamma.insert(tc::SemOcc {ident: "x".to_string(), label: 2}, empty_base());
	let pi: tc::Pi = tc::Pi { p: Vec::new() };
	let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption: Vec::new()};
	let tc_t: tc::Type = _tc.type_check();


	let mut delta: Vec<SemOcc> = Vec::new();
	delta.push(tc::SemOcc {ident: "y".to_string(), label: 3});
	let base = tc::Type::Base { delta , kappa: Vec::new() };
	
	assert!(tc_t != base);
}

#[test]
fn var_err_label_occ()
{
	let occurrence = "(x)".to_string();

	let expr: exprParser::Expr = exprParser::parser(occurrence);
	let occ = convExprToOcc::convert(expr);
	
	let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
	gamma.insert(tc::SemOcc {ident: "x".to_string(), label: 2}, empty_base());
	let pi: tc::Pi = tc::Pi { p: Vec::new() };
	let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption: Vec::new()};
	let tc_t: tc::Type = _tc.type_check();


	let mut delta: Vec<SemOcc> = Vec::new();
	delta.push(tc::SemOcc {ident: "x".to_string(), label: 3});
	let base = tc::Type::Base { delta , kappa: Vec::new() };
	
	assert!(tc_t != base);
}

#[test]
fn var_err_ident_occ()
{
	let occurrence = "(x)".to_string();

	let expr: exprParser::Expr = exprParser::parser(occurrence);
	let occ = convExprToOcc::convert(expr);
	
	let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
	gamma.insert(tc::SemOcc {ident: "x".to_string(), label: 2}, empty_base());
	let pi: tc::Pi = tc::Pi { p: Vec::new() };
	let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption: Vec::new()};
	let tc_t: tc::Type = _tc.type_check();


	let mut delta: Vec<SemOcc> = Vec::new();
	delta.push(tc::SemOcc {ident: "y".to_string(), label: 2});
	let base = tc::Type::Base { delta , kappa: Vec::new() };
	
	assert!(tc_t != base);
}

#[test]
fn all_pi_simple()
{
	let mut p: Vec<(usize, usize)> = Vec::new();
	p.push((0,1));
	p.push((1,2));

	let pi: tc::Pi = tc::Pi { p };
	let res: Vec<tc::Pi> = pi.clone().get_all_pi(2);

	println!("{:#?}", pi);

	assert!(res == vec![pi]);
}
