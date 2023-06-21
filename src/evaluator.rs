use crate::occParser;
use crate::occParser::Constant;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
pub enum Val
{
    Const(Constant),
    Closure 
    {
        ident: String,
        body: Box<occParser::Occ>,
        penv: HashMap<String, Val>,
    },
    RClosure 
    {
        name: String,
        identR: String,
        bodyR: Box<occParser::Occ>,
        penvR: HashMap<String, Val>,
    },
    Loc(String),
    Unit,
}

static mut next: usize = 0;

pub fn intepret(occ: occParser::Occ) -> Val
{
    let mut env: HashMap<String, Val> = HashMap::new();
    let mut sto: HashMap<String, Val> = HashMap::new();

    let mut v: Val;
    (v,sto) = eval(occ, env, sto);
    return v;
}

fn eval(occ: occParser::Occ, mut env: HashMap<String, Val>, mut sto: HashMap<String, Val>) -> (Val, HashMap<String, Val>)
{
    let expr: occParser::Expr = occ.expr;

    match expr.ExpType
    {
        occParser::Type::Const =>
        {
            if expr.ident.parse::<usize>().is_ok() { return (Val::Const(Constant::Num(expr.ident.parse::<usize>().unwrap())), sto); }
            else if expr.ident == "true".to_string() { return (Val::Const(Constant::Bool(true)), sto); }
            else if expr.ident == "false".to_string() { return (Val::Const(Constant::Bool(false)), sto); }
            else { unreachable!(); }
        }

        occParser::Type::Var =>
        {
            let val = env.get(&expr.ident);
            match val
            {
                Some(v) =>
                {
                    return ((*v).clone(), sto);
                }
                None => unreachable!()
            }
        }

        occParser::Type::Fun =>
        {
            let e: Box<occParser::Occ>;
            let mut p_env: HashMap<String, Val> = HashMap::new();
            p_env.extend(env.clone());

            match expr.LHS { Some(ee) => { e = ee } None => unreachable!() }
            return (Val::Closure{ident: expr.ident, body: e, penv: p_env}, sto);
        }

        occParser::Type::App =>
        {
            match expr.LHS 
            { 
                Some(e) => 
                {
                    let v: Val;
                    (v, sto) = eval(Box::into_inner(e),env.clone(),sto);
                    match v
                    {
                        Val::Closure { ident, body, mut penv } => 
                        {
                            let mut v2: Val;
                            match expr.RHS { Some(e) => { (v2, sto) = eval(Box::into_inner(e),env.clone(),sto) } None => unreachable!() }
                            penv.insert(ident.clone(), v2);

                            (v2, sto) = eval(Box::into_inner(body), penv.clone(),sto);
                            
                            return (v2, sto);
                        }
                        Val::RClosure { name, identR, bodyR, mut penvR } => 
                        {
                            let mut v2: Val;
                            match expr.RHS { Some(e) => { (v2, sto) = eval(Box::into_inner(e),env.clone(),sto) } None => unreachable!() }
                            penvR.insert(identR.clone(), v2.clone());
                            let vr: Val = Val::RClosure { name: name.clone(), identR: identR.clone(), bodyR: bodyR.clone(), penvR: penvR.clone() };
                            penvR.insert(name.clone(), vr);

                            (v2, sto) = eval(Box::into_inner(bodyR), penvR.clone(),sto);
                            
                            return (v2, sto);
                        }
                        _=> unreachable!()
                    }
                } 
                None => unreachable!() 
            }
        }


        occParser::Type::FApp =>
        {
            let v1: Val;
            let v2: Val;
            match expr.LHS { Some(e) => { (v1, sto) = eval(Box::into_inner(e),env.clone(),sto) } None => unreachable!() }
            match expr.RHS { Some(e) => { (v2, sto) = eval(Box::into_inner(e),env.clone(),sto) } None => unreachable!() }

            match expr.ident.as_str()
            {
                "PLUS" =>
                {
                    let c1: Constant;
                    let c2: Constant;
                    match v1 { Val::Const(c) => { c1 = c} _=>unreachable!() }
                    match v2 { Val::Const(c) => { c2 = c} _=>unreachable!() }
                    return (Val::Const(c1 + c2),sto)
                }
                "MINUS" =>
                {
                    let c1: Constant;
                    let c2: Constant;
                    match v1 { Val::Const(c) => { c1 = c} _=>unreachable!() }
                    match v2 { Val::Const(c) => { c2 = c} _=>unreachable!() }
                    return (Val::Const(c1 - c2),sto)
                }
                "TIMES" =>
                {
                    let c1: Constant;
                    let c2: Constant;
                    match v1 { Val::Const(c) => { c1 = c} _=>unreachable!() }
                    match v2 { Val::Const(c) => { c2 = c} _=>unreachable!() }
                    return (Val::Const(c1 * c2),sto)
                }
                "GREATER" =>
                {
                    let c1: Constant;
                    let c2: Constant;
                    match v1 { Val::Const(c) => { c1 = c} _=>unreachable!() }
                    match v2 { Val::Const(c) => { c2 = c} _=>unreachable!() }
                    return (Val::Const(occParser::Great(c1, c2)),sto)
                }
                "LESS" =>
                {
                    let c1: Constant;
                    let c2: Constant;
                    match v1 { Val::Const(c) => { c1 = c} _=>unreachable!() }
                    match v2 { Val::Const(c) => { c2 = c} _=>unreachable!() }
                    return (Val::Const(occParser::Less(c1, c2)),sto)
                }
                "EQUAL" =>
                {
                    let c1: Constant;
                    let c2: Constant;
                    match v1 { Val::Const(c) => { c1 = c} _=>unreachable!() }
                    match v2 { Val::Const(c) => { c2 = c} _=>unreachable!() }
                    return (Val::Const(occParser::Equal(c1, c2)),sto)
                }
                "NEQUAL" =>
                {
                    let c1: Constant;
                    let c2: Constant;
                    match v1 { Val::Const(c) => { c1 = c} _=>unreachable!() }
                    match v2 { Val::Const(c) => { c2 = c} _=>unreachable!() }
                    return (Val::Const(occParser::NEqual(c1, c2)),sto)
                }
                "LEQ" =>
                {
                    let c1: Constant;
                    let c2: Constant;
                    match v1 { Val::Const(c) => { c1 = c} _=>unreachable!() }
                    match v2 { Val::Const(c) => { c2 = c} _=>unreachable!() }
                    return (Val::Const(occParser::LEq(c1, c2)),sto)
                }
                "GEQ" =>
                {
                    let c1: Constant;
                    let c2: Constant;
                    match v1 { Val::Const(c) => { c1 = c} _=>unreachable!() }
                    match v2 { Val::Const(c) => { c2 = c} _=>unreachable!() }
                    return (Val::Const(occParser::GEq(c1, c2)),sto)
                }
                _=>{ return (v1,sto) }
            }
        }
        
        occParser::Type::Let =>
        {
            let mut v: Val;
            let var = expr.ident.clone();
            match expr.LHS { Some(e) => { (v, sto) = eval(Box::into_inner(e),env.clone(),sto) } None => unreachable!() }
            env.insert(var.clone(), v);
            match expr.RHS { Some(e) => { (v, sto) = eval(Box::into_inner(e),env.clone(),sto) } None => unreachable!() }
            env.remove(&(var));

            return (v, sto);
        }
        
        occParser::Type::LetR =>
        {
            match expr.LHS 
            { 
                Some(e) => 
                {
                    let v: Val;
                    (v, sto) = eval(Box::into_inner(e),env.clone(),sto);
                    match v
                    {
                        Val::Closure { ident, body, penv } => 
                        {
                            let var = expr.ident.clone();
                            let rval: Val = Val::RClosure { name: var.clone(), identR: ident.clone(), bodyR: body.clone(), penvR: penv.clone() };

                            env.insert(var.clone(), rval);
                            let v2: Val;
                            match expr.RHS { Some(e) => { (v2, sto) = eval(Box::into_inner(e),env.clone(),sto) } None => unreachable!() }
                            env.remove(&(var));
                            
                            return (v2, sto);
                        }
                        _=> unreachable!()
                    }
                } 
                None => unreachable!() 
            }
        }
        
        occParser::Type::Case =>
        {
            let mut v: Val;
            let i: usize;
            let p: occParser::Pat;
            match expr.LHS { Some(e) => { (v, sto) = eval(Box::into_inner(e),env.clone(),sto) } None => unreachable!() }
            match expr.Pats { Some(e) => { i = p_match(v.clone(), e.clone()); p = e[i].clone(); } None => unreachable!() }
            match expr.Occs { Some(e) => 
                {
                    match p
                    {
                        occParser::Pat::Var(x) =>
                        {
                            env.insert(x.clone(), v.clone());
                            (v, sto) = eval(Box::into_inner(e[i].clone()),env.clone(), sto);
                            env.remove(&(x));
                        }
                        _=> { (v, sto) = eval(Box::into_inner(e[i].clone()),env.clone(), sto); }
                    }
                } 
                None => unreachable!() }

            return (v, sto);
        }

        occParser::Type::Ref =>
        {
            let v: Val;
            match expr.LHS { Some(e) => { (v, sto) = eval(Box::into_inner(e),env.clone(), sto) } None => unreachable!() }
            let loc: String;
            unsafe { loc = "_".to_string() + &next.to_string(); next = next+1; }
            sto.insert(loc.clone(), v);
            return (Val::Loc(loc), sto);
        }

        occParser::Type::RefW =>
        {
            let l: Val;
            let v: Val;
            match expr.LHS { Some(e) => { (l, sto) = eval(Box::into_inner(e),env.clone(), sto) } None => unreachable!() }
            match expr.RHS { Some(e) => { (v, sto) = eval(Box::into_inner(e),env.clone(), sto) } None => unreachable!() }
            match l
            {
                Val::Loc(loc) =>
                {
                    sto.insert(loc.clone(), v);
                    return (Val::Unit, sto);
                }
                _=> { unreachable!(); }
            }
        }

        occParser::Type::RefR =>
        {
            let v: Val;
            match expr.LHS { Some(e) => { (v, sto) = eval(Box::into_inner(e),env.clone(), sto) } None => unreachable!() }
            
            match v
            {
                Val::Loc(loc) =>
                {
                    let res = sto.get(&loc);
                    match res { Some(val) => { return ((*val).clone(), sto); } None => unreachable!() }
                }
                _=> { unreachable!(); }
            }
        }

        _=>
        {
            println!("Unimplemented rule");
            return (Val::Const(Constant::Num(0)), sto);
        }
    }
}

fn p_match(v: Val, pats: Vec<occParser::Pat>) -> usize
{
    match v
    {
        Val::Const(x) =>
        {
            for i in 0..pats.len()
            {
                match pats[i].clone()
                {
                    occParser::Pat::Const(y) =>
                    {
                        if y == x { return i; }
                    }
                    _=> { return i; }
                }
            }
            unreachable!();
        }
        _=> { unreachable!(); }
    }
}
