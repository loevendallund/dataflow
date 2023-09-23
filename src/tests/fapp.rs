use std::collections::HashMap;

use crate::occParser;
use crate::exprParser;
use crate::convExprToOcc;
use crate::evaluator;
use crate::tc;
use crate::tc::SemOcc;

fn empty_base() -> tc::Type { tc::Type::Base { delta: Vec::new(), kappa: Vec::new() } }

#[test]
fn const_functional_app()
{
    let occurrence = "(PLUS (10) (20))".to_string();

    let expr: exprParser::Expr = exprParser::parser(occurrence);
    let occ = convExprToOcc::convert(expr);

    let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    let pi: tc::Pi = tc::Pi { p: Vec::new() };
    let mut assumption = Vec::new();

    let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption};
    let tc_t: tc::Type = _tc.type_check();

    let base = tc::Type::Base { delta: Vec::new() , kappa: Vec::new() };

    assert!(tc_t == base)
}

#[test]
fn var_left_functional_app()
{
    let occurrence = "(PLUS (x) (20))".to_string();

    let expr: exprParser::Expr = exprParser::parser(occurrence);
    let occ = convExprToOcc::convert(expr);

    let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    let pi: tc::Pi = tc::Pi { p: Vec::new() };
    let mut assumption = Vec::new();

    gamma.insert(tc::SemOcc {ident: "x".to_string(), label: 2}, empty_base());

    let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption};
    let tc_t: tc::Type = _tc.type_check();

    let mut delta: Vec<SemOcc> = Vec::new();
    delta.push(tc::SemOcc {ident: "x".to_string(), label: 1});

    let base = tc::Type::Base { delta , kappa: Vec::new() };

    assert!(tc_t == base)
}

#[test]
fn var_right_functional_app()
{
    let occurrence = "(PLUS (10) (x))".to_string();

    let expr: exprParser::Expr = exprParser::parser(occurrence);
    let occ = convExprToOcc::convert(expr);

    let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    let pi: tc::Pi = tc::Pi { p: Vec::new() };
    let mut assumption = Vec::new();

    gamma.insert(tc::SemOcc {ident: "x".to_string(), label: 3}, empty_base());

    let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption};
    let tc_t: tc::Type = _tc.type_check();

    let mut delta: Vec<SemOcc> = Vec::new();
    delta.push(tc::SemOcc {ident: "x".to_string(), label: 2});

    let base = tc::Type::Base { delta , kappa: Vec::new() };

    assert!(tc_t == base)
}

#[test]
fn var_both_functional_app()
{
    let occurrence = "(PLUS (x) (y))".to_string();

    let expr: exprParser::Expr = exprParser::parser(occurrence);
    let occ = convExprToOcc::convert(expr);

    let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    let pi: tc::Pi = tc::Pi { p: Vec::new() };
    let mut assumption = Vec::new();

    gamma.insert(tc::SemOcc {ident: "x".to_string(), label: 3}, empty_base());
    gamma.insert(tc::SemOcc {ident: "y".to_string(), label: 3}, empty_base());

    let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption};
    let tc_t: tc::Type = _tc.type_check();

    let mut delta: Vec<SemOcc> = Vec::new();
    delta.push(tc::SemOcc {ident: "x".to_string(), label: 1});
    delta.push(tc::SemOcc {ident: "y".to_string(), label: 2});

    let base = tc::Type::Base { delta , kappa: Vec::new() };

    assert!(tc_t == base)
}
