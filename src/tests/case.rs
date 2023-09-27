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
fn case_simple()
{
    let occurrence = "(case (true) (true)((10)))".to_string();

    let expr: exprParser::Expr = exprParser::parser(occurrence);
    let occ = convExprToOcc::convert(expr);

    let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    let pi = approx(occ.clone());
    let mut assumption = Vec::new();

    let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption};
    let tc_t: tc::Type = _tc.type_check();

    let mut delta: Vec<tc::SemOcc> = Vec::new();
    let mut kappa: Vec<String> = Vec::new();

    let base = tc::Type::Base { delta, kappa };

    let mut gamma1: HashMap<tc::SemOcc, tc::Type> = HashMap::new();

    println!("New: {:#?}",tc_t);
    println!("{:#?}",base);

    assert!(tc_t == base);
    assert!(_tc.gamma.len() == gamma1.len());
    for (key, val) in _tc.gamma.clone()
    {
        assert!(gamma1.contains_key(&key));
        match gamma1.get(&key)
        {
            Some(t) => { assert!(*t == val); }
            None => { assert!(false); }
        }
    }
}

#[test]
fn case_simple_condition()
{
    let occurrence = "(case (LESS (1) (2)) (true,false)((10),(20)))".to_string();

    let expr: exprParser::Expr = exprParser::parser(occurrence);
    let occ = convExprToOcc::convert(expr);

    let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    let pi = approx(occ.clone());
    let mut assumption = Vec::new();

    let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption};
    let tc_t: tc::Type = _tc.type_check();

    let mut delta: Vec<tc::SemOcc> = Vec::new();
    let mut kappa: Vec<String> = Vec::new();

    let base = tc::Type::Base { delta, kappa };

    let mut gamma1: HashMap<tc::SemOcc, tc::Type> = HashMap::new();

    println!("New: {:#?}",tc_t);
    println!("{:#?}",base);

    assert!(tc_t == base);
    assert!(_tc.gamma.len() == gamma1.len());
    for (key, val) in _tc.gamma.clone()
    {
        assert!(gamma1.contains_key(&key));
        match gamma1.get(&key)
        {
            Some(t) => { assert!(*t == val); }
            None => { assert!(false); }
        }
    }
}

#[test]
fn case_simple_condition_var_1()
{
    let occurrence = "(case (LESS (1) (2)) (true,false)((x),(20)))".to_string();

    let expr: exprParser::Expr = exprParser::parser(occurrence);
    let occ = convExprToOcc::convert(expr);

    let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    let pi = approx(occ.clone());
    let mut assumption = Vec::new();

    gamma.insert(tc::SemOcc {ident: "x".to_string(), label: 5}, tc::Type::Base { delta: Vec::new(), kappa: Vec::new() });

    let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption};
    let tc_t: tc::Type = _tc.type_check();

    let mut delta: Vec<tc::SemOcc> = Vec::new();
    let mut kappa: Vec<String> = Vec::new();

    delta.push(tc::SemOcc {ident: "x".to_string(), label: 4});

    let base = tc::Type::Base { delta, kappa };

    let mut gamma1: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    gamma1.insert(tc::SemOcc {ident: "x".to_string(), label: 5}, tc::Type::Base { delta: Vec::new(), kappa: Vec::new() });

    println!("New: {:#?}",tc_t);
    println!("{:#?}",base);

    assert!(tc_t == base);
    assert!(_tc.gamma.len() == gamma1.len());
    for (key, val) in _tc.gamma.clone()
    {
        assert!(gamma1.contains_key(&key));
        match gamma1.get(&key)
        {
            Some(t) => { assert!(*t == val); }
            None => { assert!(false); }
        }
    }
}

#[test]
fn case_simple_condition_var_2()
{
    let occurrence = "(case (LESS (1) (2)) (true,false)((10),(x)))".to_string();

    let expr: exprParser::Expr = exprParser::parser(occurrence);
    let occ = convExprToOcc::convert(expr);

    let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    let pi = approx(occ.clone());
    let mut assumption = Vec::new();

    gamma.insert(tc::SemOcc {ident: "x".to_string(), label: 5}, tc::Type::Base { delta: Vec::new(), kappa: Vec::new() });

    let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption};
    let tc_t: tc::Type = _tc.type_check();

    let mut delta: Vec<tc::SemOcc> = Vec::new();
    let mut kappa: Vec<String> = Vec::new();

    delta.push(tc::SemOcc {ident: "x".to_string(), label: 5});

    let base = tc::Type::Base { delta, kappa };

    let mut gamma1: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    gamma1.insert(tc::SemOcc {ident: "x".to_string(), label: 5}, tc::Type::Base { delta: Vec::new(), kappa: Vec::new() });

    println!("New: {:#?}",tc_t);
    println!("{:#?}",base);

    assert!(tc_t == base);
    assert!(_tc.gamma.len() == gamma1.len());
    for (key, val) in _tc.gamma.clone()
    {
        assert!(gamma1.contains_key(&key));
        match gamma1.get(&key)
        {
            Some(t) => { assert!(*t == val); }
            None => { assert!(false); }
        }
    }
}
