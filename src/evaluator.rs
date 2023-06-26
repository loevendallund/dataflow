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

#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
pub struct SOcc
{
    pub name: String,
    pub label: usize,
}

static mut next: usize = 0;

pub fn intepret(occ: occParser::Occ) -> (Val, HashMap<SOcc, (Vec<SOcc>, Vec<SOcc>)>, HashMap<String, usize>, (Vec<SOcc>, Vec<SOcc>))
{
    let mut env: HashMap<String, Val> = HashMap::new();
    let mut sto: HashMap<String, Val> = HashMap::new();

    let mut w: HashMap<SOcc, (Vec<SOcc>, Vec<SOcc>)> = HashMap::new();
    let mut gbind: HashMap<String, usize> = HashMap::new();

    let mut v: Val;
    let mut L: Vec<SOcc>;
    let mut V: Vec<SOcc>;

    (v,sto, w, gbind, (L, V)) = eval(occ, env, sto, w, gbind);
    return (v, w, gbind, (L, V));
}

fn eval(occ: occParser::Occ, mut env: HashMap<String, Val>, mut sto: HashMap<String, Val>, mut w: HashMap<SOcc, (Vec<SOcc>, Vec<SOcc>)>, mut gbind: HashMap<String, usize>) -> (Val, HashMap<String, Val>, HashMap<SOcc, (Vec<SOcc>, Vec<SOcc>)>, HashMap<String, usize>, (Vec<SOcc>, Vec<SOcc>))
{
    let expr: occParser::Expr = occ.expr.clone();

    match expr.ExpType
    {
        occParser::Type::Const =>
        {
            let mut L: Vec<SOcc> = Vec::new();
            let mut V: Vec<SOcc> = Vec::new();
            if expr.ident.parse::<usize>().is_ok() { return (Val::Const(Constant::Num(expr.ident.parse::<usize>().unwrap())), sto, w, gbind, (L, V)); }
            else if expr.ident == "true".to_string() { return (Val::Const(Constant::Bool(true)), sto, w, gbind, (L, V)); }
            else if expr.ident == "false".to_string() { return (Val::Const(Constant::Bool(false)), sto, w, gbind, (L, V)); }
            else { unreachable!(); }
        }

        occParser::Type::Var =>
        {
            let val = env.get(&expr.ident);
            match val
            {
                Some(v) =>
                {
                    let mut L: Vec<SOcc> = Vec::new();
                    let mut V: Vec<SOcc> = Vec::new();
                    V.push(SOcc { name: expr.ident.clone(), label:  occ.label.clone() });

                    let mut _L1: Vec<SOcc>;
                    let mut _V1: Vec<SOcc>;
                    let L1: &mut Vec<SOcc>;
                    let V1: &mut Vec<SOcc>;

                    let lab: Option<&usize> = gbind.get(&expr.ident.clone());
                    match lab 
                    { 
                        Some(v) => 
                        { 
                            let l_occ: Option<&(Vec<SOcc>, Vec<SOcc>)> = w.get(&SOcc { name: expr.ident.clone(), label: *v });

                            match l_occ { Some(v) => { (_L1, _V1) = (*v).clone();  } None => { unreachable!() }}
                            L1 = &mut _L1; V1 = &mut _V1;
                        }
                        None => { unreachable!(); }
                    }
                    L.append(L1); V.append(V1);
                    
                    return ((*v).clone(), sto, w, gbind, (L, V));
                }
                None => unreachable!()
            }
        }

        occParser::Type::Fun =>
        {
            let mut L: Vec<SOcc> = Vec::new();
            let mut V: Vec<SOcc> = Vec::new();

            let e: Box<occParser::Occ>;
            let mut p_env: HashMap<String, Val> = HashMap::new();
            p_env.extend(env.clone());

            match expr.LHS { Some(ee) => { e = ee } None => unreachable!() }
            return (Val::Closure{ident: expr.ident, body: e, penv: p_env}, sto, w, gbind, (L, V));
        }

        occParser::Type::App =>
        {
            match expr.LHS 
            { 
                Some(e) => 
                {
                    let mut L: Vec<SOcc>;
                    let mut V: Vec<SOcc>;
                    let v: Val;

                    (v, sto, w, gbind, (L, V)) = eval(Box::into_inner(e),env.clone(),sto, w, gbind);
                    match v
                    {
                        Val::Closure { ident, body, mut penv } => 
                        {
                            let mut _L1: Vec<SOcc>;
                            let mut _V1: Vec<SOcc>;
                            let mut v2: Val;
                            let lab: usize;
                            match expr.RHS { Some(e) => { lab = Box::into_inner(e.clone()).label; (v2, sto, w, gbind, (_L1, _V1)) = eval(Box::into_inner(e),env.clone(),sto, w, gbind) } None => unreachable!() }
                            penv.insert(ident.clone(), v2);
                            w.insert(SOcc { name: ident.clone(), label: lab.clone() }, (_L1.clone(),_V1.clone()));
                            gbind.insert(ident.clone(), lab.clone());
                            let L1: &mut Vec<SOcc> = &mut _L1;
                            let V1: &mut Vec<SOcc> = &mut _V1;

                            let mut _L2: Vec<SOcc>;
                            let mut _V2: Vec<SOcc>;
                            (v2, sto, w, gbind, (_L2, _V2)) = eval(Box::into_inner(body), penv.clone(),sto, w, gbind);
                            let L2: &mut Vec<SOcc> = &mut _L2;
                            let V2: &mut Vec<SOcc> = &mut _V2;

                            L.append(L1); L.append(L2);
                            V.append(V1); V.append(V2);
                            return (v2, sto, w, gbind, (L, V));
                        }
                        Val::RClosure { name, identR, bodyR, mut penvR } => 
                        {
                            let mut _L1: Vec<SOcc>;
                            let mut _V1: Vec<SOcc>;
                            let mut v2: Val;
                            let lab: usize;
                            match expr.RHS { Some(e) => { lab = Box::into_inner(e.clone()).label; (v2, sto, w, gbind, (_L1, _V1)) = eval(Box::into_inner(e),env.clone(),sto, w, gbind) } None => unreachable!() }

                            w.insert(SOcc { name: identR.clone(), label: lab.clone() }, (_L1.clone(),_V1.clone()));
                            gbind.insert(identR.clone(), lab.clone());
                            
                            penvR.insert(identR.clone(), v2.clone());
                            let vr: Val = Val::RClosure { name: name.clone(), identR: identR.clone(), bodyR: bodyR.clone(), penvR: penvR.clone() };
                            penvR.insert(name.clone(), vr);

                            let mut _L2: Vec<SOcc>;
                            let mut _V2: Vec<SOcc>;
                            (v2, sto, w, gbind, (_L2, _V2)) = eval(Box::into_inner(bodyR), penvR.clone(),sto, w, gbind);

                            let L1: &mut Vec<SOcc> = &mut _L1; let L2: &mut Vec<SOcc> = &mut _L2;
                            let V1: &mut Vec<SOcc> = &mut _V1; let V2: &mut Vec<SOcc> = &mut _V2;
    
                            L.append(L1); L.append(L2);
                            V.append(V1); V.append(V2);
                            return (v2, sto, w, gbind, (L, V));
                        }
                        _=> unreachable!()
                    }
                } 
                None => unreachable!() 
            }
        }


        occParser::Type::FApp =>
        {
            let mut L: Vec<SOcc>;
            let mut V: Vec<SOcc>;
            let mut _L1: Vec<SOcc>;
            let mut _V1: Vec<SOcc>;

            let v1: Val;
            let v2: Val;
            match expr.LHS { Some(e) => { (v1, sto, w, gbind, (L, V)) = eval(Box::into_inner(e),env.clone(),sto, w, gbind) } None => unreachable!() }
            match expr.RHS { Some(e) => { (v2, sto, w, gbind, (_L1, _V1)) = eval(Box::into_inner(e),env.clone(),sto, w, gbind) } None => unreachable!() }
            let L1: &mut Vec<SOcc> = &mut _L1;
            let V1: &mut Vec<SOcc> = &mut _V1;
            L.append(L1);
            V.append(V1);

            match expr.ident.as_str()
            {
                "PLUS" =>
                {
                    let c1: Constant;
                    let c2: Constant;
                    match v1 { Val::Const(c) => { c1 = c} _=>unreachable!() }
                    match v2 { Val::Const(c) => { c2 = c} _=>unreachable!() }
                    return (Val::Const(c1 + c2),sto, w, gbind, (L, V))
                }
                "MINUS" =>
                {
                    let c1: Constant;
                    let c2: Constant;
                    match v1 { Val::Const(c) => { c1 = c} _=>unreachable!() }
                    match v2 { Val::Const(c) => { c2 = c} _=>unreachable!() }
                    return (Val::Const(c1 - c2),sto, w, gbind, (L, V))
                }
                "TIMES" =>
                {
                    let c1: Constant;
                    let c2: Constant;
                    match v1 { Val::Const(c) => { c1 = c} _=>unreachable!() }
                    match v2 { Val::Const(c) => { c2 = c} _=>unreachable!() }
                    return (Val::Const(c1 * c2),sto, w, gbind, (L, V))
                }
                "GREATER" =>
                {
                    let c1: Constant;
                    let c2: Constant;
                    match v1 { Val::Const(c) => { c1 = c} _=>unreachable!() }
                    match v2 { Val::Const(c) => { c2 = c} _=>unreachable!() }
                    return (Val::Const(occParser::Great(c1, c2)),sto, w, gbind, (L, V))
                }
                "LESS" =>
                {
                    let c1: Constant;
                    let c2: Constant;
                    match v1 { Val::Const(c) => { c1 = c} _=>unreachable!() }
                    match v2 { Val::Const(c) => { c2 = c} _=>unreachable!() }
                    return (Val::Const(occParser::Less(c1, c2)),sto, w, gbind, (L, V))
                }
                "EQUAL" =>
                {
                    let c1: Constant;
                    let c2: Constant;
                    match v1 { Val::Const(c) => { c1 = c} _=>unreachable!() }
                    match v2 { Val::Const(c) => { c2 = c} _=>unreachable!() }
                    return (Val::Const(occParser::Equal(c1, c2)),sto, w, gbind, (L, V))
                }
                "NEQUAL" =>
                {
                    let c1: Constant;
                    let c2: Constant;
                    match v1 { Val::Const(c) => { c1 = c} _=>unreachable!() }
                    match v2 { Val::Const(c) => { c2 = c} _=>unreachable!() }
                    return (Val::Const(occParser::NEqual(c1, c2)),sto, w, gbind, (L, V))
                }
                "LEQ" =>
                {
                    let c1: Constant;
                    let c2: Constant;
                    match v1 { Val::Const(c) => { c1 = c} _=>unreachable!() }
                    match v2 { Val::Const(c) => { c2 = c} _=>unreachable!() }
                    return (Val::Const(occParser::LEq(c1, c2)),sto, w, gbind, (L, V))
                }
                "GEQ" =>
                {
                    let c1: Constant;
                    let c2: Constant;
                    match v1 { Val::Const(c) => { c1 = c} _=>unreachable!() }
                    match v2 { Val::Const(c) => { c2 = c} _=>unreachable!() }
                    return (Val::Const(occParser::GEq(c1, c2)),sto, w, gbind, (L, V))
                }
                _=>{ return (v1,sto, w, gbind, (L, V)) }
            }
        }
        
        occParser::Type::Let =>
        {
            let mut L: Vec<SOcc>;
            let mut V: Vec<SOcc>;
            let mut L1: Vec<SOcc>;
            let mut V1: Vec<SOcc>;

            let mut v: Val;
            let var = expr.ident.clone();
            
            let lab: usize;
            match expr.LHS { Some(e) => { lab = Box::into_inner(e.clone()).label; (v, sto, w, gbind, (L, V)) = eval(Box::into_inner(e),env.clone(),sto, w, gbind) } None => unreachable!() }
            env.insert(var.clone(), v);
            w.insert(SOcc { name: var.clone(), label: lab.clone() }, (L,V));
            gbind.insert(var.clone(), lab.clone());
            match expr.RHS { Some(e) => { (v, sto, w, gbind, (L1, V1)) = eval(Box::into_inner(e),env.clone(),sto, w, gbind) } None => unreachable!() }
            env.remove(&(var));

            return (v, sto, w, gbind, (L1, V1));
        }
        
        occParser::Type::LetR =>
        {
            let mut L: Vec<SOcc>;
            let mut V: Vec<SOcc>;

            match expr.LHS 
            { 
                Some(e) => 
                {
                    let v: Val;
                    let lab: usize = Box::into_inner(e.clone()).label;
                    (v, sto, w, gbind, (L, V)) = eval(Box::into_inner(e),env.clone(),sto, w, gbind);
                    match v
                    {
                        Val::Closure { ident, body, penv } => 
                        {
                            let mut L1: Vec<SOcc>;
                            let mut V1: Vec<SOcc>;
                            let var = expr.ident.clone();
                            let rval: Val = Val::RClosure { name: var.clone(), identR: ident.clone(), bodyR: body.clone(), penvR: penv.clone() };

                            env.insert(var.clone(), rval);
                            let v2: Val;
                            w.insert(SOcc { name: var.clone(), label: lab.clone() }, (L,V));
                            gbind.insert(var.clone(), lab.clone());
                            match expr.RHS { Some(e) => { (v2, sto, w, gbind, (L1, V1)) = eval(Box::into_inner(e),env.clone(),sto, w, gbind) } None => unreachable!() }
                            env.remove(&(var));
                            
                            return (v2, sto, w, gbind, (L1, V1));
                        }
                        _=> unreachable!()
                    }
                } 
                None => unreachable!() 
            }
        }
        
        occParser::Type::Case =>
        {
            let mut L: Vec<SOcc>;
            let mut V: Vec<SOcc>;

            let mut v: Val;
            let i: usize;
            let p: occParser::Pat;
            match expr.LHS { Some(e) => { (v, sto, w, gbind, (L, V)) = eval(Box::into_inner(e),env.clone(),sto, w, gbind) } None => unreachable!() }
            match expr.Pats { Some(e) => { i = p_match(v.clone(), e.clone()); p = e[i].clone(); } None => unreachable!() }
            match expr.Occs { Some(e) => 
                {
                    match p
                    {
                        occParser::Pat::Var(x) =>
                        {
                            let mut _L1: Vec<SOcc>;
                            let mut _V1: Vec<SOcc>;
                            
                            env.insert(x.clone(), v.clone());
                            (v, sto, w, gbind, (_L1, _V1)) = eval(Box::into_inner(e[i].clone()),env.clone(), sto, w, gbind);
                            env.remove(&(x));

                            let L1: &mut Vec<SOcc> = &mut _L1;
                            let V1: &mut Vec<SOcc> = &mut _V1;

                            L.append(L1); V.append(V1);
                            return (v, sto, w, gbind, (L, V));
                        }
                        _=> 
                        {
                            let mut _L1: Vec<SOcc>;
                            let mut _V1: Vec<SOcc>;
                            (v, sto, w, gbind, (_L1, _V1)) = eval(Box::into_inner(e[i].clone()),env.clone(), sto, w, gbind);

                            let L1: &mut Vec<SOcc> = &mut _L1;
                            let V1: &mut Vec<SOcc> = &mut _V1;
                            L.append(L1); V.append(V1);
                            return (v, sto, w, gbind, (L, V));
                        }
                    }
                } 
                None => unreachable!() }
        }

        occParser::Type::Ref =>
        {
            let mut L: Vec<SOcc> = Vec::new();
            let mut V: Vec<SOcc> = Vec::new();

            let v: Val;
            match expr.LHS { Some(e) => { (v, sto, w, gbind, (L, V)) = eval(Box::into_inner(e),env.clone(), sto, w, gbind) } None => unreachable!() }
            let loc: String;
            unsafe { loc = "_nu".to_string() + &next.to_string(); next = next+1; }
            sto.insert(loc.clone(), v);
            w.insert(SOcc { name: loc.clone(), label: occ.label.clone() }, (L,V));
            gbind.insert(loc.clone(), occ.label.clone());
            return (Val::Loc(loc), sto, w, gbind, (Vec::new(), Vec::new()));
        }

        occParser::Type::RefW =>
        {
            let mut L: Vec<SOcc> = Vec::new();
            let mut V: Vec<SOcc> = Vec::new();
            let mut L1: Vec<SOcc> = Vec::new();
            let mut V1: Vec<SOcc> = Vec::new();

            let l: Val;
            let v: Val;
            match expr.LHS { Some(e) => { (l, sto, w, gbind, (L, V)) = eval(Box::into_inner(e),env.clone(), sto, w, gbind) } None => unreachable!() }
            match expr.RHS { Some(e) => { (v, sto, w, gbind, (L1, V1)) = eval(Box::into_inner(e),env.clone(), sto, w, gbind) } None => unreachable!() }
            match l
            {
                Val::Loc(loc) =>
                {
                    w.insert(SOcc { name: loc.clone(), label: occ.label.clone() }, (L1,V1));
                    gbind.insert(loc.clone(), occ.label.clone());
                    sto.insert(loc.clone(), v);
                    return (Val::Unit, sto, w, gbind, (L, V));
                }
                _=> { unreachable!(); }
            }
        }

        occParser::Type::RefR =>
        {
            let mut L: Vec<SOcc> = Vec::new();
            let mut V: Vec<SOcc> = Vec::new();

            let v: Val;
            match expr.LHS { Some(e) => { (v, sto, w, gbind, (L, V)) = eval(Box::into_inner(e),env.clone(), sto, w, gbind) } None => unreachable!() }
            
            match v
            {
                Val::Loc(loc) =>
                {
                    let mut _L1: Vec<SOcc>;
                    let mut _V1: Vec<SOcc>;
                    let L1: &mut Vec<SOcc>;
                    let V1: &mut Vec<SOcc>;

                    let lab: Option<&usize> = gbind.get(&loc.clone());
                    match lab 
                    { 
                        Some(v) => 
                        { 
                            let l_occ: Option<&(Vec<SOcc>, Vec<SOcc>)> = w.get(&SOcc { name: loc.clone(), label: *v });

                            match l_occ { Some(v) => { (_L1, _V1) = (*v).clone();  } None => { unreachable!() }}
                            L1 = &mut _L1; V1 = &mut _V1;
                        }
                        None => { unreachable!(); }
                    }


                    let res = sto.get(&loc);
                    L.push(SOcc { name: loc.clone(), label: occ.label.clone() });
                    L.append(L1); V.append(V1);
                    match res { Some(val) => { return ((*val).clone(), sto, w, gbind, (L, V)); } None => unreachable!() }
                }
                _=> { unreachable!(); }
            }
        }

        _=>
        {
            println!("Unimplemented rule");
            let mut L: Vec<SOcc> = Vec::new();
            let mut V: Vec<SOcc> = Vec::new();

                    /*
                    match lab 
                    { 
                        Some(v) => 
                        { 
                            let l_occ: Option<&(Vec<SOcc>, Vec<SOcc>)> = w.get(&SOcc { name: expr.ident.clone(), label: *v });

                            match l_occ { Some(v) => { (_L1, _V1) = (*v).clone();  } None => { unreachable!() }}
                            L1 = &mut _L1; V1 = &mut _V1;
                        }
                        None => { unreachable!(); }
                    }
            L.append(L1); V.append(V1);*/
            return (Val::Const(Constant::Num(0)), sto, w, gbind, (L, V));
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
