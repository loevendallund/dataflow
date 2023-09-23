use std::collections::HashMap;

use crate::occParser;
use crate::exprParser;
use crate::convExprToOcc;
use crate::evaluator;
use crate::tc;
use crate::tc::SemOcc;

#[test]
fn const_empty_func()
{
    let occurrence = "(func x.(1))".to_string();

	let expr: exprParser::Expr = exprParser::parser(occurrence);
	let occ = convExprToOcc::convert(expr);

	let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
	let pi: tc::Pi = tc::Pi { p: Vec::new() };
   let mut assumption = Vec::new();

   assumption.push((0, tc::Type::Base { delta: Vec::new(), kappa: Vec::new() }));

	let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption};
	let tc_t: tc::Type = _tc.type_check();

	let T1 = Some(Box::new(tc::Type::Base { delta: Vec::new() , kappa: Vec::new() }));
	let T2 = Some(Box::new(tc::Type::Base { delta: Vec::new() , kappa: Vec::new() }));
	let abs = tc::Type::Abs { T1, T2 };

   assert!(tc_t == abs)
}

#[test]
fn identity_empty_func()
{
    let occurrence = "(func x.(x))".to_string();

    let expr: exprParser::Expr = exprParser::parser(occurrence);
    let occ = convExprToOcc::convert(expr);

    let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    let pi: tc::Pi = tc::Pi { p: Vec::new() };
    let mut assumption = Vec::new();

    assumption.push((0, tc::Type::Base { delta: Vec::new(), kappa: Vec::new() }));

    let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption};
    let tc_t: tc::Type = _tc.type_check();

    let mut delta: Vec<SemOcc> = Vec::new();
    delta.push(tc::SemOcc {ident: "x".to_string(), label: 1});

    let T1 = Some(Box::new(tc::Type::Base { delta: Vec::new() , kappa: Vec::new() }));
    let T2 = Some(Box::new(tc::Type::Base { delta , kappa: Vec::new() }));
    let abs = tc::Type::Abs { T1, T2 };

    assert!(tc_t == abs)
}

#[test]
fn const_func()
{
    let occurrence = "(func x.(1))".to_string();

	let expr: exprParser::Expr = exprParser::parser(occurrence);
	let occ = convExprToOcc::convert(expr);

	let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
	let pi: tc::Pi = tc::Pi { p: Vec::new() };
   let mut assumption = Vec::new();

	let mut assumpt_delta: Vec<SemOcc> = Vec::new();
	assumpt_delta.push(tc::SemOcc {ident: "y".to_string(), label: 3});
   assumption.push((0, tc::Type::Base { delta: assumpt_delta, kappa: Vec::new() }));

	let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption};
	let tc_t: tc::Type = _tc.type_check();

    let mut delta: Vec<SemOcc> = Vec::new();
    delta.push(tc::SemOcc {ident: "y".to_string(), label: 3});

    let T1 = Some(Box::new(tc::Type::Base { delta , kappa: Vec::new() }));
    let T2 = Some(Box::new(tc::Type::Base { delta: Vec::new() , kappa: Vec::new() }));
    let abs = tc::Type::Abs { T1, T2 };

    assert!(tc_t == abs)
}

#[test]
fn identity_func()
{
    let occurrence = "(func x.(x))".to_string();

	 let expr: exprParser::Expr = exprParser::parser(occurrence);
    let occ = convExprToOcc::convert(expr);

    let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    let pi: tc::Pi = tc::Pi { p: Vec::new() };
    let mut assumption = Vec::new();

    let mut assumpt_delta: Vec<SemOcc> = Vec::new();
    assumpt_delta.push(tc::SemOcc {ident: "y".to_string(), label: 3});
    assumption.push((0, tc::Type::Base { delta: assumpt_delta, kappa: Vec::new() }));

    let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption};
    let tc_t: tc::Type = _tc.type_check();

    let mut delta1: Vec<SemOcc> = Vec::new();
    let mut delta2: Vec<SemOcc> = Vec::new();

    delta1.push(tc::SemOcc {ident: "y".to_string(), label: 3});
    delta2.push(tc::SemOcc {ident: "y".to_string(), label: 3});
    delta2.push(tc::SemOcc {ident: "x".to_string(), label: 1});

    let T1 = Some(Box::new(tc::Type::Base { delta: delta1.clone() , kappa: Vec::new() }));
    let T2 = Some(Box::new(tc::Type::Base { delta: delta2.clone() , kappa: Vec::new() }));
    let abs = tc::Type::Abs { T1, T2 };

    assert!(tc_t == abs)
}
