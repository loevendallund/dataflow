use std::collections::HashMap;

use crate::occParser;
use crate::exprParser;
use crate::convExprToOcc;
use crate::evaluator;
use crate::tc;
use crate::tc::SemOcc;
use crate::approxOrder::approx;

fn empty_base() -> tc::Type { tc::Type::Base { delta: Vec::new(), kappa: Vec::new() } }

#[test]
fn const_functional_app()
{
    let occurrence = "(let x (1) (1))".to_string();

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
fn var_functional_app()
{
    let occurrence = "(let x (1) (x))".to_string();

    let expr: exprParser::Expr = exprParser::parser(occurrence);
    let occ = convExprToOcc::convert(expr);

    let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    let pi: tc::Pi = tc::Pi { p: Vec::new() };
    let mut assumption = Vec::new();

    let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption};
    let tc_t: tc::Type = _tc.type_check();

    let mut delta: Vec<SemOcc> = Vec::new();
    delta.push(tc::SemOcc {ident: "x".to_string(), label: 2});

    let base = tc::Type::Base { delta , kappa: Vec::new() };

    assert!(tc_t == base)
}

#[test]
fn var_empty_var_functional_app()
{
    let occurrence = "(let x (y) (x))".to_string();

    let expr: exprParser::Expr = exprParser::parser(occurrence);
    let occ = convExprToOcc::convert(expr);

    let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    let pi: tc::Pi = tc::Pi { p: Vec::new() };
    let mut assumption = Vec::new();

    gamma.insert(tc::SemOcc {ident: "y".to_string(), label: 5}, empty_base());

    let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption};
    let tc_t: tc::Type = _tc.type_check();

    let mut delta: Vec<SemOcc> = Vec::new();
    delta.push(tc::SemOcc {ident: "y".to_string(), label: 1});
    delta.push(tc::SemOcc {ident: "x".to_string(), label: 2});

    let base = tc::Type::Base { delta , kappa: Vec::new() };

    assert!(tc_t == base)
}


#[test]
fn var_var_functional_app()
{
    let occurrence = "(let x (y) (x))".to_string();

    let expr: exprParser::Expr = exprParser::parser(occurrence);
    let occ = convExprToOcc::convert(expr);

    let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    let pi: tc::Pi = tc::Pi { p: Vec::new() };
    let mut assumption = Vec::new();

    let mut delta1: Vec<SemOcc> = Vec::new();
    delta1.push(tc::SemOcc {ident: "z".to_string(), label: 6});

    gamma.insert(tc::SemOcc {ident: "y".to_string(), label: 5}, tc::Type::Base { delta: delta1, kappa: Vec::new() });

    let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption};
    let tc_t: tc::Type = _tc.type_check();

    let mut delta: Vec<SemOcc> = Vec::new();
    delta.push(tc::SemOcc {ident: "z".to_string(), label: 6});
    delta.push(tc::SemOcc {ident: "y".to_string(), label: 1});
    delta.push(tc::SemOcc {ident: "x".to_string(), label: 2});

    let base = tc::Type::Base { delta , kappa: Vec::new() };

    assert!(tc_t == base)
}


#[test]
fn var_loc_simple_functional_app()
{
    let occurrence = "(let x (ref (1)) (x))".to_string();

    let expr: exprParser::Expr = exprParser::parser(occurrence);
    let occ = convExprToOcc::convert(expr);

    let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    let pi: tc::Pi = tc::Pi { p: Vec::new() };
    let mut assumption = Vec::new();

    let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption};
    let tc_t: tc::Type = _tc.type_check();

    let mut delta: Vec<SemOcc> = Vec::new();
    delta.push(tc::SemOcc {ident: "x".to_string(), label: 3});
    let mut kappa: Vec<String> = Vec::new();
    kappa.push("_nu0".to_string());

    let base = tc::Type::Base { delta , kappa };

    assert!(tc_t == base)
}

#[test]
fn var_loc_complex_functional_app()
{
    let occurrence = "(let x (ref (1)) (!(x)))".to_string();

    let expr: exprParser::Expr = exprParser::parser(occurrence);
    let occ = convExprToOcc::convert(expr);

    let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    let pi = approx(occ.clone());
    let mut assumption = Vec::new();

    let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption};
    let tc_t: tc::Type = _tc.type_check();

    let mut delta: Vec<SemOcc> = Vec::new();
    delta.push(tc::SemOcc {ident: "x".to_string(), label: 3});
    delta.push(tc::SemOcc {ident: "_nu0".to_string(), label: 2});
    let mut kappa: Vec<String> = Vec::new();

    let base = tc::Type::Base { delta , kappa };

    assert!(tc_t == base)
}
