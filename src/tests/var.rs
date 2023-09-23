use std::collections::HashMap;

use crate::occParser;
use crate::exprParser;
use crate::convExprToOcc;
use crate::evaluator;
use crate::tc;
use crate::tc::SemOcc;

fn empty_base() -> tc::Type { tc::Type::Base { delta: Vec::new(), kappa: Vec::new() } }

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
