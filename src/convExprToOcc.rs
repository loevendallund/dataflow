use crate::occParser;
use crate::occParser::Expr;
use crate::occParser::Type;
use crate::occParser::Occ;

use crate::exprParser;


pub fn convert(expression: exprParser::Expr) -> Occ
{
    let occ: Occ;
    (occ,_) = _convert(expression, 1);
    return occ;
}

fn _convert(expression: exprParser::Expr, mut index: usize) -> (Occ, usize)
{
    let eType = expression.ExpType;
    let mut oExpr: Expr = Expr { ExpType: eType.clone(), ident: expression.ident, LHS: None, RHS: None, Pats: expression.Pats, Occs: None };
    match eType 
    {
        Type::Const =>
        {
            return (Occ{ expr: oExpr, label: index, }, index+1);
        }

        Type::Var =>
        {
            return (Occ{ expr: oExpr, label: index, }, index+1);
        }

        Type::Fun =>
        {
            let e: Occ;
            match expression.LHS { Some(lhs) => { (e,index) = _convert(Box::into_inner(lhs), index); oExpr.LHS = Some(Box::new(e)); } None => {unreachable!()} }
            return (Occ{ expr: oExpr, label: index, }, index+1);
        }

        Type::App =>
        {
            let e1: Occ;
            let e2: Occ;
            match expression.LHS { Some(lhs) => { (e1,index) = _convert(Box::into_inner(lhs), index); oExpr.LHS = Some(Box::new(e1)); } None => {unreachable!()} }
            match expression.RHS{ Some(rhs) => { (e2,index) = _convert(Box::into_inner(rhs), index); oExpr.RHS = Some(Box::new(e2)); } None => {unreachable!()} }
            return (Occ{ expr: oExpr, label: index, }, index+1);
        }

        Type::FApp =>
        {
            let e1: Occ;
            let e2: Occ;
            match expression.LHS { Some(lhs) => { (e1,index) = _convert(Box::into_inner(lhs), index); oExpr.LHS = Some(Box::new(e1)); } None => {unreachable!()} }
            match expression.RHS{ Some(rhs) => { (e2,index) = _convert(Box::into_inner(rhs), index); oExpr.RHS = Some(Box::new(e2)); } None => {unreachable!()} }
            return (Occ{ expr: oExpr, label: index, }, index+1);
        }

        Type::Let =>
        {
            let e1: Occ;
            let e2: Occ;
            match expression.LHS { Some(lhs) => { (e1,index) = _convert(Box::into_inner(lhs), index); oExpr.LHS = Some(Box::new(e1)); } None => {unreachable!()} }
            match expression.RHS{ Some(rhs) => { (e2,index) = _convert(Box::into_inner(rhs), index); oExpr.RHS = Some(Box::new(e2)); } None => {unreachable!()} }
            return (Occ{ expr: oExpr, label: index, }, index+1);
        }

        Type::LetR =>
        {
            let e1: Occ;
            let e2: Occ;
            match expression.LHS { Some(lhs) => { (e1,index) = _convert(Box::into_inner(lhs), index); oExpr.LHS = Some(Box::new(e1)); } None => {unreachable!()} }
            match expression.RHS{ Some(rhs) => { (e2,index) = _convert(Box::into_inner(rhs), index); oExpr.RHS = Some(Box::new(e2)); } None => {unreachable!()} }
            return (Occ{ expr: oExpr, label: index, }, index+1);
        }

        Type::Case =>
        {
            let e1: Occ;
            match expression.LHS { Some(lhs) => { (e1,index) = _convert(Box::into_inner(lhs), index); oExpr.LHS = Some(Box::new(e1)); } None => {unreachable!()} }
            let ovec: Vec<Box<Occ>>;
            match expression.Exprs { Some(evec) => { (ovec,index) = case_occs(evec, index); oExpr.Occs = Some(ovec); } None => {unreachable!()} }
            return (Occ{ expr: oExpr, label: index, }, index+1);
        }

        Type::Ref =>
        {
            let e1: Occ;
            match expression.LHS { Some(lhs) => { (e1,index) = _convert(Box::into_inner(lhs), index); oExpr.LHS = Some(Box::new(e1)); } None => {unreachable!()} }
            return (Occ{ expr: oExpr, label: index, }, index+1);
        }

        Type::RefW =>
        {
            let e1: Occ;
            let e2: Occ;
            match expression.LHS { Some(lhs) => { (e1,index) = _convert(Box::into_inner(lhs), index); oExpr.LHS = Some(Box::new(e1)); } None => {unreachable!()} }
            match expression.RHS{ Some(rhs) => { (e2,index) = _convert(Box::into_inner(rhs), index); oExpr.RHS = Some(Box::new(e2)); } None => {unreachable!()} }
            return (Occ{ expr: oExpr, label: index, }, index+1);
        }

        Type::RefR =>
        {
            let e1: Occ;
            match expression.LHS { Some(lhs) => { (e1,index) = _convert(Box::into_inner(lhs), index); oExpr.LHS = Some(Box::new(e1)); } None => {unreachable!()} }
            return (Occ{ expr: oExpr, label: index, }, index+1);
        }

        _=>
        {
            return ( Occ{ expr: oExpr, label: index, }, index+1);
        }
    }
}

fn case_occs(mut evec: Vec<Box<exprParser::Expr>>, mut index: usize) -> (Vec<Box<Occ>>, usize)
{
    let mut ovec: Vec<Box<Occ>> = Vec::new();
    for e in evec
    {
        let oelement: Occ;
        (oelement, index) = _convert(Box::into_inner(e),index);
        ovec.push(Box::new(oelement));
    }
    return (ovec,index);
}
