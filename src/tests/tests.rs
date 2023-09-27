use std::collections::HashMap;

use crate::occParser;
use crate::exprParser;
use crate::convExprToOcc;
use crate::evaluator;
use crate::tc;
use crate::tc::SemOcc;

#[cfg(test)] pub mod var;
#[cfg(test)] pub mod constants;
#[cfg(test)] pub mod func;
#[cfg(test)] pub mod app;
#[cfg(test)] pub mod fapp;
#[cfg(test)] pub mod letTest;
#[cfg(test)] pub mod case;
#[cfg(test)] pub mod reference;
#[cfg(test)] pub mod reference_read;
#[cfg(test)] pub mod reference_write;

fn empty_base() -> tc::Type { tc::Type::Base { delta: Vec::new(), kappa: Vec::new() } }

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
