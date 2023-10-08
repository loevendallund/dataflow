use std::collections::HashMap;

use crate::occParser::Occ;
use crate::occParser::Type;

use crate::tc::Pi;

pub fn approx(occ: Occ) -> Pi
{
    let vec: Vec<(usize,usize)>;
    (vec,_,_) = ApproximateOrder(occ, 0);
    let pi: Pi = Pi {p: vec};
    return pi;
}

fn ApproximateOrder(occ: Occ, last_point: usize) -> (Vec<(usize,usize)>, usize, Option<Box<Occ>>)
{
    match occ.expr.ExpType
    {
        Type::Const =>	{ approx_end(occ, last_point) }
        Type::Var   =>	{ approx_end(occ, last_point) }
        Type::Fun   =>	{ approx_abs(occ, last_point) }
        Type::App   =>	{ approx_app(occ, last_point) }
        Type::FApp  =>	{ approx_none(occ, last_point) }
        Type::Let   =>	{ approx_let(occ, last_point) }
        Type::Case  =>  { approx_complex(occ, last_point) }
        Type::Ref   =>	{ approx_ref(occ, last_point) }
        Type::RefR  =>	{ approx_ref(occ, last_point) }
        Type::RefW  =>	{ approx_none(occ, last_point) }
        _ => { unreachable!("Either those doesn't work or are unimplemented!!!!"); }
		}
}

fn approx_end(occ: Occ, last_point: usize) -> (Vec<(usize,usize)>, usize, Option<Box<Occ>>)
{
    let mut vec: Vec<(usize,usize)> = Vec::new();
    vec.push((last_point, occ.label.clone()));
    return (vec, occ.label, None);
}

fn approx_abs(occ: Occ, last_point: usize) -> (Vec<(usize,usize)>, usize, Option<Box<Occ>>)
{
    let mut vec: Vec<(usize,usize)> = Vec::new();
    vec.push((last_point, occ.label.clone()));
    return (vec, occ.label, occ.expr.RHS.clone());
}

fn approx_app(occ: Occ, last_point: usize) -> (Vec<(usize,usize)>, usize, Option<Box<Occ>>)
{
    let mut vec1: Vec<(usize,usize)> = Vec::new();
    let p1: usize;
    let l_occ: Option<Box<Occ>>;
    match occ.expr.LHS { Some(a) => { (vec1,p1,l_occ) = ApproximateOrder(Box::into_inner(a.clone()), last_point); vec1.push((last_point.clone(), a.label.clone())); } None => { unreachable!(); } }

    let mut vec2: Vec<(usize,usize)> = Vec::new();
    let p2: usize;

    match occ.expr.RHS { Some(a) => { (vec2,p2,_) = ApproximateOrder(Box::into_inner(a.clone()), p1); vec2.push((last_point.clone(), a.label.clone())); } None => { unreachable!(); } }

    let mut l_vec: Vec<(usize,usize)> = Vec::new();
    let l_p: usize;
    let res_occ: Option<Box<Occ>>;
    match l_occ { Some(a) => { (l_vec,l_p,res_occ) = ApproximateOrder(Box::into_inner(a.clone()), p2); l_vec.push((last_point.clone(), a.label.clone())); } None => { unreachable!("Expected left hand sign of application to be an abstraction"); } }

    vec1.extend(vec2);
    vec1.extend(l_vec);
    //vec1.push((p2, occ.label.clone()));
    vec1.push((l_p, occ.label.clone()));

    return (vec1, occ.label, res_occ);
}

fn approx_none(occ: Occ, last_point: usize) -> (Vec<(usize,usize)>, usize, Option<Box<Occ>>)
{
    let mut vec1: Vec<(usize,usize)> = Vec::new();
    let p1: usize;
    match occ.expr.LHS { Some(a) => { (vec1,p1,_) = ApproximateOrder(Box::into_inner(a.clone()), last_point); vec1.push((last_point.clone(), a.label.clone())); } None => { unreachable!(); } }

    let mut vec2: Vec<(usize,usize)> = Vec::new();
    let p2: usize;
    match occ.expr.RHS { Some(a) => { (vec2,p2,_) = ApproximateOrder(Box::into_inner(a.clone()), p1); vec2.push((last_point.clone(), a.label.clone())); } None => { unreachable!(); } }

    vec1.extend(vec2);
    vec1.push((p2, occ.label.clone()));

    return (vec1, occ.label, None);
}

fn approx_let(occ: Occ, last_point: usize) -> (Vec<(usize,usize)>, usize, Option<Box<Occ>>)
{
    let mut vec1: Vec<(usize,usize)> = Vec::new();
    let p1: usize;
    match occ.expr.LHS { Some(a) => { (vec1,p1,_) = ApproximateOrder(Box::into_inner(a.clone()), last_point); vec1.push((last_point.clone(), a.label.clone())); } None => { unreachable!(); } }

    let mut vec2: Vec<(usize,usize)> = Vec::new();
    let p2: usize;
    let res_occ: Option<Box<Occ>>;
    match occ.expr.RHS { Some(a) => { (vec2,p2,res_occ) = ApproximateOrder(Box::into_inner(a.clone()), p1); vec2.push((last_point.clone(), a.label.clone())); } None => { unreachable!(); } }

    vec1.extend(vec2);
    vec1.push((p2, occ.label.clone()));

    return (vec1, occ.label, res_occ);
}

fn approx_complex(occ: Occ, last_point: usize) -> (Vec<(usize,usize)>, usize, Option<Box<Occ>>)
{
    let mut vec1: Vec<(usize,usize)> = Vec::new();
    let p1: usize;
    match occ.expr.LHS { Some(a) => { (vec1,p1,_) = ApproximateOrder(Box::into_inner(a.clone()), last_point); vec1.push((last_point.clone(), a.label.clone())); } None => { unreachable!(); } }

    let occs: Vec<Box<Occ>>;
    occs = match occ.expr.Occs.clone() { Some(occs) => { occs } None => {unreachable!()} };

    let mut res_occ: Option<Box<Occ>> = None;
    for _occ in occs
    {
        let mut vec2: Vec<(usize,usize)>;
        let p2: usize;
        (vec2,p2,res_occ) = ApproximateOrder(Box::into_inner(_occ.clone()), p1);

        vec1.extend(vec2.into_iter());
        vec1.push((p2, occ.label.clone()));
    }

    return (vec1, occ.label, res_occ);
}

fn approx_ref(occ: Occ, last_point: usize) -> (Vec<(usize,usize)>, usize, Option<Box<Occ>>)
{
    let mut vec1: Vec<(usize,usize)> = Vec::new();
    let p1: usize;
    match occ.expr.LHS { Some(a) => { (vec1,p1,_) = ApproximateOrder(Box::into_inner(a.clone()), last_point); vec1.push((last_point.clone(), a.label.clone())); } None => { unreachable!(); } }

    vec1.push((p1, occ.label.clone()));

    return (vec1, occ.label, None);
}

/* Old way of approximating
pub fn old_approx(occ:Occ) -> HashMap<usize,usize>
{
    let mut Pi: HashMap<usize, usize>;
    let p: usize;
    (Pi, p) = ApproxOrder(occ, 0);
    return Pi;
}

fn ApproxOrder(occ: Occ, last_point: usize) -> (HashMap<usize,usize>, usize)
{
    match occ.expr.ExpType
    {
        Type::Const =>
        { 
            let mut h: HashMap<usize,usize> = HashMap::new();
            h.insert(last_point, occ.label.clone());
            return (h, occ.label); 
        }
        Type::Var =>
        { 
            let mut h: HashMap<usize,usize> = HashMap::new();
            h.insert(last_point, occ.label.clone());
            return (h, occ.label); 
        }

        /*Type::Fun =>
        {
            match occ.expr.LHS
        }*/

        Type::FApp =>
        {
            let mut h1: HashMap<usize,usize>;
            let p1: usize;
            match occ.expr.LHS { Some(a) => { (h1,p1) = ApproxOrder(Box::into_inner(a.clone()), last_point); h1.insert(last_point.clone(), a.label.clone()); } None => { unreachable!(); } }

            let h2: HashMap<usize,usize>;
            let p2: usize;
            match occ.expr.RHS { Some(a) => { (h2,p2) = ApproxOrder(Box::into_inner(a), p1.clone()); } None => { unreachable!(); } }

            h1.extend(h2.into_iter());
            h1.insert(p2, occ.label.clone());

            return (h1, occ.label.clone());
        }

        Type::Let =>
        {
            let mut h1: HashMap<usize,usize>;
            let p1: usize;
            match occ.expr.LHS { Some(a) => { (h1,p1) = ApproxOrder(Box::into_inner(a.clone()), last_point); h1.insert(last_point.clone(), a.label.clone()); } None => { unreachable!(); } }

            let h2: HashMap<usize,usize>;
            let p2: usize;
            match occ.expr.RHS { Some(a) => { (h2,p2) = ApproxOrder(Box::into_inner(a), p1.clone()); } None => { unreachable!(); } }

            h1.extend(h2.into_iter());
            h1.insert(p2, occ.label.clone());

            return (h1, occ.label.clone());
        }

        Type::Case =>
        {
            let mut h1: HashMap<usize,usize>;
            let p1: usize;
            match occ.expr.LHS { Some(a) => { (h1,p1) = ApproxOrder(Box::into_inner(a.clone()), last_point); h1.insert(last_point.clone(), a.label.clone()); } None => { unreachable!(); } }

            let occs: Vec<Box<Occ>>;
            occs = match occ.expr.Occs.clone() { Some(occs) => { occs } None => {unreachable!()} };
            for _occ in occs
            {
                let mut h2: HashMap<usize,usize>;
                let p2: usize;
                (h2,p2) = ApproxOrder(Box::into_inner(_occ.clone()), p1);
                
                h1.extend(h2.into_iter());
                h1.insert(p2, occ.label.clone());
            }

            return (h1, occ.label.clone());
        }

        Type::Ref =>
        {
            let mut h1: HashMap<usize,usize>;
            let p1: usize;
            match occ.expr.LHS { Some(a) => { (h1,p1) = ApproxOrder(Box::into_inner(a.clone()), last_point); h1.insert(last_point.clone(), a.label.clone()); } None => { unreachable!(); } }

            h1.insert(p1, occ.label.clone());

            return (h1, occ.label.clone());
        }

        Type::RefW =>
        {
            let mut h1: HashMap<usize,usize>;
            let p1: usize;
            match occ.expr.LHS { Some(a) => { (h1,p1) = ApproxOrder(Box::into_inner(a.clone()), last_point); h1.insert(last_point.clone(), a.label.clone()); } None => { unreachable!(); } }

            let h2: HashMap<usize,usize>;
            let p2: usize;
            match occ.expr.RHS { Some(a) => { (h2,p2) = ApproxOrder(Box::into_inner(a), p1.clone()); } None => { unreachable!(); } }

            h1.extend(h2.into_iter());
            h1.insert(p2, occ.label.clone());

            return (h1, occ.label.clone());
        }

        Type::RefR =>
        {
            let mut h1: HashMap<usize,usize>;
            let p1: usize;
            match occ.expr.LHS { Some(a) => { (h1,p1) = ApproxOrder(Box::into_inner(a.clone()), last_point); h1.insert(last_point.clone(), a.label.clone()); } None => { unreachable!(); } }

            h1.insert(p1, occ.label.clone());

            return (h1, occ.label.clone());
        }

        _=> { println!("Unimplemented"); return (HashMap::new(), 0); }
    }
}
*/
