use crate::occParser::Type;
use crate::occParser::Pat;
use crate::occParser::Constant;

use pest::{Parser, iterators::{Pairs, Pair}};
use std::fs;

#[derive(Parser)]
#[grammar = "parser/exprGrammar.pest"]
struct ExprParser;

#[derive(Debug)]
#[derive(Clone)]
pub struct Expr
{
    pub ExpType: Type,
    pub ident: String,
    pub LHS: Option<Box<Expr>>,
    pub RHS: Option<Box<Expr>>,
    pub Pats: Option<Vec<Pat>>,
    pub Exprs: Option<Vec<Box<Expr>>>,
}

pub fn parser(mut str: String) -> Expr
{
    let Parsed = ExprParser::parse(Rule::file, &str)
        .expect("unsuccessful parse")
        .next().unwrap();

    let mut expression = Expr { ExpType: Type::NONE, ident: "".to_string(), LHS: None, RHS: None, Pats: None, Exprs: None, };

    for line in Parsed.into_inner() 
    {
        match line.as_rule()
        {
            Rule::expr =>
            {
                expression = parse(line.into_inner());
            }
            Rule::EOI => (), _=> unreachable!(),
        }
    }
    return expression;
}

fn parse(mut pairs: Pairs<Rule>) -> Expr
{
    let mut expr: Expr;

    let mut rule = pairs.next().unwrap();

    match rule.as_rule()
    {
        Rule::constant => {
            let mut inner = rule.into_inner();
            expr = Expr{
                ExpType: Type::Const,
                ident: inner.next().unwrap().as_str().to_string(),
                LHS: None,
                RHS: None,
                Pats: None,
                Exprs: None,
            };
        }

        Rule::ident => {
            expr = Expr{
                ExpType: Type::Var,
                ident: rule.clone().as_span().as_str().to_string(),
                LHS: None,
                RHS: None,
                Pats: None,
                Exprs: None,
            };
        }
        
        Rule::Fun => {
            let mut inner = rule.into_inner();
            expr = Expr{
                ExpType: Type::Fun,
                ident: inner.next().unwrap().as_str().to_string(),
                LHS: Some(Box::new(parse(inner.next().unwrap().into_inner()))),
                RHS: None,
                Pats: None,
                Exprs: None,
            };
        }
        
        Rule::App => {
            let mut inner = rule.into_inner();
            expr = Expr{
                ExpType: Type::App,
                ident: "".to_string(),
                LHS: Some(Box::new(parse(inner.next().unwrap().into_inner()))),
                RHS: Some(Box::new(parse(inner.next().unwrap().into_inner()))),
                Pats: None,
                Exprs: None,
            };
        }
        
        Rule::FApp => {
            let mut inner = rule.into_inner();
            expr = Expr{
                ExpType: Type::FApp,
                ident: inner.next().unwrap().as_str().to_string(),
                LHS: Some(Box::new(parse(inner.next().unwrap().into_inner()))),
                RHS: Some(Box::new(parse(inner.next().unwrap().into_inner()))),
                Pats: None,
                Exprs: None,
            };
        }
        
        Rule::Let => {
            let mut inner = rule.into_inner();
            expr = Expr{
                ExpType: Type::Let,
                ident: inner.next().unwrap().as_str().to_string(),
                LHS: Some(Box::new(parse(inner.next().unwrap().into_inner()))),
                RHS: Some(Box::new(parse(inner.next().unwrap().into_inner()))),
                Pats: None,
                Exprs: None,
            };
        }
        
        Rule::LetR => {
            let mut inner = rule.into_inner();
            expr = Expr{
                ExpType: Type::LetR,
                ident: inner.next().unwrap().as_str().to_string(),
                LHS: Some(Box::new(parse(inner.next().unwrap().into_inner()))),
                RHS: Some(Box::new(parse(inner.next().unwrap().into_inner()))),
                Pats: None,
                Exprs: None,
            };
        }
        
        Rule::Case => {
            let mut inner = rule.into_inner();
            let id = Some(Box::new(parse(inner.next().unwrap().into_inner())));
            let l_pats = parse_pats(inner.next().unwrap());
            let l_exprs = parse_exprs(inner.next().unwrap());
            if (l_pats.len() != l_exprs.len()) { unreachable!(); }
            expr = Expr{
                ExpType: Type::Case,
                ident: "".to_string(),
                LHS: id,
                RHS: None,//Some(Box::new(parse(inner.next().unwrap().into_inner()))),
                Pats: Some(l_pats),
                Exprs: Some(l_exprs),
            };

        }

        Rule::Ref => {
            let mut inner = rule.into_inner();
            expr = Expr{
                ExpType: Type::Ref,
                ident: "".to_string(),
                LHS: Some(Box::new(parse(inner.next().unwrap().into_inner()))),
                RHS: None,
                Pats: None,
                Exprs: None,
            };
        }
        
        Rule::RefW => {
            let mut inner = rule.into_inner();
            expr = Expr{
                ExpType: Type::RefW,
                ident: "".to_string(),
                LHS: Some(Box::new(parse(inner.next().unwrap().into_inner()))),
                RHS: Some(Box::new(parse(inner.next().unwrap().into_inner()))),
                Pats: None,
                Exprs: None,
            };
        }

        Rule::RefR => {
            let mut inner = rule.into_inner();
            expr = Expr{
                ExpType: Type::RefR,
                ident: "".to_string(),
                LHS: Some(Box::new(parse(inner.next().unwrap().into_inner()))),
                RHS: None,
                Pats: None,
                Exprs: None,
            };
        }

        _=>{
        expr = Expr{
            ExpType: Type::NONE,
            ident: "".to_string(),
            LHS: None,
            RHS: None,
            Pats: None,
            Exprs: None,
        };
        }
    }

    return expr;
}

pub fn parse_exprs(mut pair: Pair<Rule>) -> Vec<Box<Expr>>
{
    let res = exprs(pair,Vec::new());
    return res;
}

fn exprs(mut pair: Pair<Rule>, mut vec: Vec<Box<Expr>>) -> Vec<Box<Expr>>
{
    let mut inner = pair.into_inner();
    if (inner.clone().count() == 1)
    {
        vec.push(Box::new(parse(inner.next().unwrap().into_inner())));
        return vec;
    }
    else
    {
        vec.push(Box::new(parse(inner.next().unwrap().into_inner())));
        return exprs(inner.next().unwrap(),vec);
    }
}

pub fn parse_pats(mut pair: Pair<Rule>) -> Vec<Pat>
{
    let v = pats(pair, Vec::new());
    return v
}

fn pats(mut pair: Pair<Rule>, mut vec: Vec<Pat>) -> Vec<Pat>
{
    let mut inner = pair.into_inner();
    let count = inner.clone().count();
    
    let pat = conv_pat(inner.next().unwrap().as_str().to_string());
    vec.push(pat);

    if count > 1
    {
        return pats(inner.next().unwrap(), vec);
    }
    return vec;
}

fn conv_pat(pat: String) -> Pat
{
    if pat.parse::<usize>().is_ok() { return Pat::Const(Constant::Num(pat.parse::<usize>().unwrap())); }
    else if pat == "true" { return Pat::Const(Constant::Bool(true)); }
    else if pat == "false" { return Pat::Const(Constant::Bool(false)); }
    else if pat == "_" { return Pat::Wild; }
    else { return Pat::Var(pat); }
}
