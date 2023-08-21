use std::collections::HashMap;

use crate::occParser::Occ;
use crate::occParser::Type;

pub fn approx(occ:Occ) -> HashMap<usize,usize>
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
