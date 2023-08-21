use crate::occParser;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
pub struct SemOcc
{
    pub ident: String,
    pub label: usize,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Type
{
    Base {
        delta: Vec<SemOcc>,
        kappa: Vec<String>,
    },
    Abs {
        T1: Option<Box<Type>>,
        T2: Option<Box<Type>>,
    }
}

static mut next: usize = 0;

pub fn TCheck(mut Gamma: HashMap<SemOcc, Type>, Pi: HashMap<usize, usize>, occ: occParser::Occ, v: Vec<HashMap<usize,usize>>, mut assumption: Option<Type>) -> (Type, HashMap<SemOcc, Type>)
{
    match occ.expr.ExpType
    {
        occParser::Type::Const =>
        {
            return (Type::Base {
                delta: Vec::new(),
                kappa: Vec::new(),
            }, Gamma);
        }

        occParser::Type::Var =>
        {
            let lookup: SemOcc = GetBinding(occ.clone().expr.ident, Gamma.clone(), Pi);
            let mut t: Type = match Gamma.get(&lookup) { Some(val) => { (*val).clone() } None => { unreachable!() }};
            t = sqUnion(t, vec![SemOcc { ident: occ.expr.ident, label: occ.label }]);

            return (t, Gamma);
        }

        occParser::Type::Fun =>
        {
            let t1: Type;
            let tin: Type;
            match occ.clone().expr.LHS 
            {
                Some(occ1) => 
                {
                    let _occ1 = Box::into_inner(occ1);
                    let socc = SemOcc { ident: _occ1.expr.ident.clone(), label: 0 };
                    match assumption.clone() { Some(a) => { tin=a.clone(); Gamma.insert(socc.clone(), a); } None => { unreachable!() } };
                    (t1, Gamma) = TCheck(Gamma, Pi.clone(), _occ1, v.clone(), None);
                    Gamma.remove(&socc);
                } 
                None => { unreachable!() }
            }

            return (Type::Abs { T1: Some(Box::new(tin)), T2: Some(Box::new(t1)) }, Gamma);
        }

        occParser::Type::App =>
        {
            let t2: Type;
            match occ.clone().expr.RHS { Some(occ2) => {(t2, Gamma) = TCheck(Gamma, Pi.clone(), Box::into_inner(occ2), v.clone(), assumption.clone()); } None => { unreachable!() } }

            assumption = Some(t2.clone());

            let t1: Type;
            match occ.clone().expr.LHS { Some(occ1) => {(t1, Gamma) = TCheck(Gamma, Pi.clone(), Box::into_inner(occ1), v.clone(), assumption.clone()); } None => { unreachable!() } }
            //println!("assumption: {:#?}", assumption);
            //println!("type: {:#?}", t1);

            match t1 { 
                Type::Abs { T1, T2 } =>  
                {
                    match T1 { Some(a) => { if !BaseEqual(Box::into_inner(a), t2) { unreachable!() } } None => {unreachable!() } }
                    match T2 { Some(a) => { return (Box::into_inner(a), Gamma) } None => {unreachable!() } }
                }
                _ => { unreachable!(); }
            }
        }

        occParser::Type::Let =>
        {
            let t1: Type;
            match occ.clone().expr.LHS { Some(occ1) => {(t1, Gamma) = TCheck(Gamma, Pi.clone(), Box::into_inner(occ1), v.clone(), assumption.clone()); } None => { unreachable!() } }
            let key = SemOcc { ident: occ.clone().expr.ident, label: occ.clone().label };
            Gamma.insert(key.clone(), t1);

            let t2: Type;
            match occ.clone().expr.RHS { Some(occ2) => {(t2, Gamma) = TCheck(Gamma, Pi.clone(), Box::into_inner(occ2), v, assumption.clone()); } None => { unreachable!() } }
            Gamma.remove(&key);

            return (t2, Gamma);
        }

        occParser::Type::FApp =>
        {
            let t1: Type;
            match occ.clone().expr.LHS { Some(occ1) => {(t1, Gamma) = TCheck(Gamma, Pi.clone(), Box::into_inner(occ1), v.clone(), assumption.clone()); } None => { unreachable!() } }

            let t2: Type;
            match occ.clone().expr.RHS { Some(occ2) => {(t2, Gamma) = TCheck(Gamma, Pi.clone(), Box::into_inner(occ2), v, assumption.clone()); } None => { unreachable!() } }

            return (Union(t1, t2), Gamma);
        }

        occParser::Type::Case =>
        {
            let t1: Type;
            let _lab: usize;
            match occ.clone().expr.LHS { Some(occ1) => {(t1, Gamma) = TCheck(Gamma.clone(), Pi.clone(), Box::into_inner(occ1.clone()), v.clone(), assumption.clone()); _lab = occ1.label; } None => { unreachable!() } }

            let pats: Vec<occParser::Pat>;
            pats = match occ.clone().expr.Pats { Some(pats) => { pats } None => {unreachable!()} };

            let occs: Vec<Box<occParser::Occ>>;
            occs = match occ.clone().expr.Occs { Some(occs) => { occs } None => {unreachable!()} };

            let mut t: Option<Type> = None;
            for i in 0..pats.len()
            {
                let mut _x: String = "".to_string();
                let p = &pats[i];
                match (*p).clone() { occParser::Pat::Var(x) => { _x = x.clone(); Gamma.insert(SemOcc { ident: x, label: _lab.clone() }, t1.clone()); } _=>{} }
                let o = &occs[i];
                let (ti, mut gamma) = TCheck(Gamma.clone(), Pi.clone(), Box::into_inner((*o).clone()), v.clone(), assumption.clone());
                Gamma = gamma;
                if _x != "" { Gamma.remove(&SemOcc { ident: _x, label: _lab.clone() }); }

                match t {
                    Some(_t) => { t = Some(Union(_t, ti)) }
                    None => { t = Some(ti); }
                }
            }

            match t { Some(_t) => { return (Union(_t, t1), Gamma) } None => { unreachable!() }};
        }

        occParser::Type::Ref =>
        {
            let t1: Type;
            match occ.clone().expr.LHS { Some(occ1) => {(t1, Gamma) = TCheck(Gamma, Pi.clone(), Box::into_inner(occ1), v, assumption.clone()); } None => { unreachable!() } }

            let k: Vec<String>;
            unsafe 
            {
                let id = "_nu".to_string() + &next.to_string();
                Gamma.insert(SemOcc { ident: id.clone() , label: occ.label.clone() }, t1);
                k = vec![id];
                next = next+1;
            }

            return (Type::Base { delta: Vec::new(), kappa: k }, Gamma);
        }

        occParser::Type::RefW =>
        {
            let t1: Type;
            match occ.clone().expr.LHS { Some(occ1) => {(t1, Gamma) = TCheck(Gamma, Pi.clone(), Box::into_inner(occ1), v.clone(), assumption.clone()); } None => { unreachable!() } }

            let t2: Type;
            match occ.clone().expr.LHS { Some(occ1) => {(t2, Gamma) = TCheck(Gamma, Pi.clone(), Box::into_inner(occ1), v, assumption.clone()); } None => { unreachable!() } }

            let d1: Vec<SemOcc>;
            match t1.clone()
            {
                Type::Base { delta, kappa } =>
                {
                    d1 = delta.clone();
                    for loc in kappa.clone()
                    {
                        if loc.contains("_nu")
                        {
                            Gamma.insert(SemOcc { ident: loc.clone(), label: occ.label.clone() }, t1.clone());
                        }
                    }
                }
                Type::Abs { T1:_, T2:_ } => { unreachable!() }               
            }

            return (Type::Base { delta: d1, kappa: Vec::new() }, Gamma);
        }

        occParser::Type::RefR =>
        {
            let t1: Type;
            match occ.clone().expr.LHS { Some(occ1) => {(t1, Gamma) = TCheck(Gamma, Pi.clone(), Box::into_inner(occ1), v.clone(), assumption.clone()); } None => { unreachable!() } }
            let key = SemOcc { ident: occ.clone().expr.ident, label: occ.clone().label };

            let mut occs: Vec<SemOcc> = Vec::new();
            let mut d1: Vec<SemOcc>;
            match t1.clone()
            {
                Type::Base { delta, kappa } =>
                {
                    d1 = delta;
                    for loc in kappa.clone()
                    {
                        if loc.contains("_nu")
                        {
                            occs = GetAllGBinding(loc, occ.clone().label, Gamma.clone(), v.clone());
                        }
                    }
                }
                Type::Abs { T1:_, T2:_ } => { unreachable!() }               
            }

            occs.append(&mut d1);

            return (Type::Base { delta: occs, kappa: Vec::new() }, Gamma);
        }

        _ =>
        {
            println!("Unimplemented: {:#?}",occ.clone().expr.ExpType);
            return (Type::Base {
                delta: Vec::new(),
                kappa: Vec::new(),
            }, Gamma);
        }
    }
}

fn GetBinding(ident: String, mut Gamma: HashMap<SemOcc, Type>, Pi: HashMap<usize, usize>) -> SemOcc
{
    let mut gBind: Option<usize> = None;
    let mut bindOcc: SemOcc = SemOcc { ident: ident.clone(), label: 0 };
    for key in Gamma.keys()
    {
        if key.ident == ident
        {
            gBind = Some(key.label.clone());
            bindOcc.label = key.label;
        }
    }
    if None == gBind { unreachable!() }
    return bindOcc;
}

fn GetAllGBinding(ident: String, point: usize, mut Gamma: HashMap<SemOcc, Type>, v: Vec<HashMap<usize,usize>>) -> Vec<SemOcc>
{
    let mut occurrences: Vec<SemOcc> = Vec::new();
    for pi in v
    {
        let mut gBind: usize = 0;
        let mut bindOcc: SemOcc = SemOcc { ident: ident.clone(), label: 0 };
        for key in Gamma.keys()
        {
            if key.ident == ident
            {
                let _v = pi.get(&gBind);
                match _v { 
                    Some(j) => 
                    {
                        if *j == key.clone().label
                        {
                            gBind = key.label;
                            bindOcc.label = gBind;
                        }
                    } 
                    None => 
                    {
                        gBind = key.label;
                        bindOcc.label = gBind;
                    }}
            }
        }
        if bindOcc.label == 0 { unreachable!() }
        occurrences.push(bindOcc);
    }

    return occurrences;
}

fn sqUnion(mut t: Type, mut b: Vec<SemOcc>) -> Type
{
    match t
    {
        Type::Base{mut delta,kappa} => 
        { 
            delta.append(&mut b); 
            let d: Vec<SemOcc> = delta;
            let k: Vec<String> = kappa;
            return Type::Base { delta: d, kappa: k};
        }
        Type::Abs { T1, T2 } =>
        {
            let t1 = T1;
            let t2: Type;
            match T2 { Some(_t) => { t2 = sqUnion(Box::into_inner(_t), b); } None => { unreachable!() }}
            return Type::Abs { T1: t1, T2: Some(Box::new(t2)) };
        }
    }
}

fn Union(mut t1: Type, mut t2: Type) -> Type
{
    match t1
    {
        Type::Base{mut delta, mut kappa} => 
        { 
            let mut d1: Vec<SemOcc> = delta;
            let mut k1: Vec<String> = kappa;
            let mut d2: Vec<SemOcc>;
            let mut k2: Vec<String>;

            (d2, k2) = match t2 { Type::Base { delta, kappa } => { (delta, kappa) } _ => { unreachable!() } };

            d1.append(&mut d2);
            k1.append(&mut k2);

            return Type::Base { delta: d1, kappa: k1};
        }
        Type::Abs { T1, T2 } =>
        {
            let t11: Type;
            let t12: Type;
            t11 = match T1 { Some(_t) => { Box::into_inner(_t) } None => { unreachable!() } };
            t12 = match T2 { Some(_t) => { Box::into_inner(_t) } None => { unreachable!() } };

            let t21: Type;
            let t22: Type;

            (t21, t22) = match t2 { Type::Abs { T1, T2 } => 
                { 
                    (
                        match T1 { Some(_t) => { Box::into_inner(_t) } None => { unreachable!() } },
                        match T2 { Some(_t) => { Box::into_inner(_t) } None => { unreachable!() } }
                    ) 
                } 
                _ => { unreachable!() }};

            return Type::Abs { T1: Some(Box::new(Union(t11, t21))), T2:Some(Box::new(Union(t12, t22))) };
        }
    }
}

fn BaseEqual(t1: Type, t2: Type) -> bool
{
    let d1: Vec<SemOcc>;
    let k1: Vec<String>;
    (d1,k1) = match t1 { Type::Base { delta, kappa } => { (delta, kappa) } _ => { unreachable!() } };

    let d2: Vec<SemOcc>;
    let k2: Vec<String>;
    (d2,k2) = match t2 { Type::Base { delta, kappa } => { (delta, kappa) } _ => { unreachable!() } };

    if d1.eq(&d2) && k1.eq(&k2) { return true; }
    return false;
}
