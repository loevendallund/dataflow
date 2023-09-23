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
fn write_empty()
{
    let occurrence = "((ref (1)):=(1))".to_string();

    let expr: exprParser::Expr = exprParser::parser(occurrence);
    let occ = convExprToOcc::convert(expr);

    let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    let pi: tc::Pi = tc::Pi { p: Vec::new() };
    let mut assumption = Vec::new();

    let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption};
    let tc_t: tc::Type = _tc.type_check();

    let mut kappa: Vec<String> = Vec::new();

    let base = tc::Type::Base { delta: Vec::new() , kappa };

    let mut gamma1: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    gamma1.insert(tc::SemOcc {ident: "_nu0".to_string(), label: 2}, tc::Type::Base { delta: Vec::new() , kappa: Vec::new() });
    gamma1.insert(tc::SemOcc {ident: "_nu0".to_string(), label: 4}, tc::Type::Base { delta: Vec::new() , kappa: Vec::new() });

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
fn write_var_empty()
{
    let occurrence = "((ref (x)):=(1))".to_string();

    let expr: exprParser::Expr = exprParser::parser(occurrence);
    let occ = convExprToOcc::convert(expr);

    let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    let pi: tc::Pi = tc::Pi { p: Vec::new() };
    let mut assumption = Vec::new();

    gamma.insert(tc::SemOcc {ident: "x".to_string(), label: 5}, tc::Type::Base { delta: Vec::new(), kappa: Vec::new() });

    let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption};
    let tc_t: tc::Type = _tc.type_check();

    let mut delta: Vec<tc::SemOcc> = Vec::new();
    let mut kappa: Vec<String> = Vec::new();

    let base = tc::Type::Base { delta, kappa };

    let mut delta1: Vec<tc::SemOcc> = Vec::new();
    delta1.push(tc::SemOcc {ident: "x".to_string(), label: 1});

    let mut gamma1: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    gamma1.insert(tc::SemOcc {ident: "x".to_string(), label: 5}, tc::Type::Base { delta: Vec::new() , kappa: Vec::new() });
    gamma1.insert(tc::SemOcc {ident: "_nu0".to_string(), label: 4}, tc::Type::Base { delta: Vec::new() , kappa: Vec::new() });
    gamma1.insert(tc::SemOcc {ident: "_nu0".to_string(), label: 2}, tc::Type::Base { delta: delta1 , kappa: Vec::new() });

    println!("{:#?}", _tc.gamma);
    println!("{:#?}", gamma1);

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
fn write_empty_var()
{
    let occurrence = "((ref (1)):=(x))".to_string();

    let expr: exprParser::Expr = exprParser::parser(occurrence);
    let occ = convExprToOcc::convert(expr);

    let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    let pi: tc::Pi = tc::Pi { p: Vec::new() };
    let mut assumption = Vec::new();

    gamma.insert(tc::SemOcc {ident: "x".to_string(), label: 5}, tc::Type::Base { delta: Vec::new(), kappa: Vec::new() });

    let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption};
    let tc_t: tc::Type = _tc.type_check();

    let mut delta: Vec<tc::SemOcc> = Vec::new();
    let mut kappa: Vec<String> = Vec::new();

    let base = tc::Type::Base { delta, kappa };

    let mut delta1: Vec<tc::SemOcc> = Vec::new();
    delta1.push(tc::SemOcc {ident: "x".to_string(), label: 3});

    let mut gamma1: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    gamma1.insert(tc::SemOcc {ident: "x".to_string(), label: 5}, tc::Type::Base { delta: Vec::new() , kappa: Vec::new() });
    gamma1.insert(tc::SemOcc {ident: "_nu0".to_string(), label: 4}, tc::Type::Base { delta: delta1 , kappa: Vec::new() });
    gamma1.insert(tc::SemOcc {ident: "_nu0".to_string(), label: 2}, tc::Type::Base { delta: Vec::new() , kappa: Vec::new() });

    println!("{:#?}", _tc.gamma);
    println!("{:#?}", gamma1);

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
fn write_var_var()
{
    let occurrence = "((ref (x)):=(y))".to_string();

    let expr: exprParser::Expr = exprParser::parser(occurrence);
    let occ = convExprToOcc::convert(expr);

    let mut gamma: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    let pi: tc::Pi = tc::Pi { p: Vec::new() };
    let mut assumption = Vec::new();

    gamma.insert(tc::SemOcc {ident: "x".to_string(), label: 5}, tc::Type::Base { delta: Vec::new(), kappa: Vec::new() });
    gamma.insert(tc::SemOcc {ident: "y".to_string(), label: 6}, tc::Type::Base { delta: Vec::new(), kappa: Vec::new() });

    let mut _tc: tc::TypeChecker = tc::TypeChecker{ gamma, pi, occ, assumption};
    let tc_t: tc::Type = _tc.type_check();

    let mut delta: Vec<tc::SemOcc> = Vec::new();
    let mut kappa: Vec<String> = Vec::new();

    let base = tc::Type::Base { delta, kappa };

    let mut delta1: Vec<tc::SemOcc> = Vec::new();
    delta1.push(tc::SemOcc {ident: "x".to_string(), label: 1});
    let mut delta2: Vec<tc::SemOcc> = Vec::new();
    delta2.push(tc::SemOcc {ident: "y".to_string(), label: 3});

    let mut gamma1: HashMap<tc::SemOcc, tc::Type> = HashMap::new();
    gamma1.insert(tc::SemOcc {ident: "x".to_string(), label: 5}, tc::Type::Base { delta: Vec::new() , kappa: Vec::new() });
    gamma1.insert(tc::SemOcc {ident: "y".to_string(), label: 6}, tc::Type::Base { delta: Vec::new() , kappa: Vec::new() });
    gamma1.insert(tc::SemOcc {ident: "_nu0".to_string(), label: 4}, tc::Type::Base { delta: delta2 , kappa: Vec::new() });
    gamma1.insert(tc::SemOcc {ident: "_nu0".to_string(), label: 2}, tc::Type::Base { delta: delta1 , kappa: Vec::new() });

    println!("{:#?}", _tc.gamma);
    println!("{:#?}", gamma1);

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
