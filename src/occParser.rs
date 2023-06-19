use pest::{Parser, iterators::{Pairs, Pair}};
use std::fs;

#[derive(Parser)]
#[grammar = "parser/occGrammar.pest"]
struct ExprParser;

#[derive(Debug)]
#[derive(Clone)]
pub struct Expr
{
    pub ExpType: Type,
    pub ident: String,
    pub LHS: Option<Box<Occ>>,
    pub RHS: Option<Box<Occ>>,
    pub Pats: Option<Vec<Pat>>,
    pub Occs: Option<Vec<Box<Occ>>>,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Occ
{
    pub expr: Expr,
    pub label: usize,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Pat
{
    Const(usize),
    Bool(bool),
    Var(String),
    Wild,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Type
{
    NONE,
    Const,
    Var,
    Fun,
    App,
    FApp,
    Let,
    LetR,
    Case,
    Ref,
    RefW,
    RefR
}

pub fn Parse_Expr(mut str: String) -> Occ
{
    let Parsed = ExprParser::parse(Rule::file, &str)
        .expect("unsuccessful parse")
        .next().unwrap();

    let mut occurrence = Occ { expr: Expr { ExpType: Type::NONE, ident: "".to_string(), LHS: None, RHS: None, Pats: None, Occs: None, }, label: 0, };

    for line in Parsed.into_inner() 
    {
        match line.as_rule()
        {
            Rule::occurrence =>
            {
                occurrence = parse_occ(line.into_inner());
            }
            Rule::EOI => (), _=> unreachable!(),
        }
    }
    return occurrence;
}

pub fn parse_occ(mut pairs: Pairs<Rule>) -> Occ
{
    let expr: Expr = parse_expr(pairs.next().unwrap());
    let label: usize = pairs.next().unwrap().as_str().parse::<usize>().unwrap();

    return Occ{expr, label};
}

fn parse_expr(mut pair: Pair<Rule>) -> Expr
{
    let mut expr: Expr;

    let mut rule = pair.into_inner().next().unwrap();

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
                Occs: None,
            };
        }

        Rule::ident => {
            expr = Expr{
                ExpType: Type::Var,
                ident: rule.clone().as_span().as_str().to_string(),
                LHS: None,
                RHS: None,
                Pats: None,
                Occs: None,
            };
        }
        
        Rule::Fun => {
            let mut inner = rule.into_inner();
            expr = Expr{
                ExpType: Type::Fun,
                ident: inner.next().unwrap().as_str().to_string(),
                LHS: Some(Box::new(parse_occ(inner.next().unwrap().into_inner()))),
                RHS: None,
                Pats: None,
                Occs: None,
            };
        }
        
        Rule::App => {
            let mut inner = rule.into_inner();
            expr = Expr{
                ExpType: Type::App,
                ident: "".to_string(),
                LHS: Some(Box::new(parse_occ(inner.next().unwrap().into_inner()))),
                RHS: Some(Box::new(parse_occ(inner.next().unwrap().into_inner()))),
                Pats: None,
                Occs: None,
            };
        }
        
        Rule::FApp => {
            let mut inner = rule.into_inner();
            expr = Expr{
                ExpType: Type::FApp,
                ident: inner.next().unwrap().as_str().to_string(),
                LHS: Some(Box::new(parse_occ(inner.next().unwrap().into_inner()))),
                RHS: Some(Box::new(parse_occ(inner.next().unwrap().into_inner()))),
                Pats: None,
                Occs: None,
            };
        }
        
        Rule::Let => {
            let mut inner = rule.into_inner();
            expr = Expr{
                ExpType: Type::Let,
                ident: inner.next().unwrap().as_str().to_string(),
                LHS: Some(Box::new(parse_occ(inner.next().unwrap().into_inner()))),
                RHS: Some(Box::new(parse_occ(inner.next().unwrap().into_inner()))),
                Pats: None,
                Occs: None,
            };
        }
        
        Rule::LetR => {
            let mut inner = rule.into_inner();
            expr = Expr{
                ExpType: Type::LetR,
                ident: inner.next().unwrap().as_str().to_string(),
                LHS: Some(Box::new(parse_occ(inner.next().unwrap().into_inner()))),
                RHS: Some(Box::new(parse_occ(inner.next().unwrap().into_inner()))),
                Pats: None,
                Occs: None,
            };
        }
        
        Rule::Case => {
            let mut inner = rule.into_inner();
            let id = Some(Box::new(parse_occ(inner.next().unwrap().into_inner())));
            let l_pats = parse_pats(inner.next().unwrap());
            let l_occs = parse_occs(inner.next().unwrap());
            if (l_pats.len() != l_occs.len()) { unreachable!(); }
            expr = Expr{
                ExpType: Type::Case,
                ident: "".to_string(),
                LHS: id,
                RHS: None,//Some(Box::new(parse_occ(inner.next().unwrap().into_inner()))),
                Pats: Some(l_pats),
                Occs: Some(l_occs),
            };

        }

        Rule::Ref => {
            let mut inner = rule.into_inner();
            expr = Expr{
                ExpType: Type::Ref,
                ident: "".to_string(),
                LHS: Some(Box::new(parse_occ(inner.next().unwrap().into_inner()))),
                RHS: None,
                Pats: None,
                Occs: None,
            };
        }
        
        Rule::RefW => {
            let mut inner = rule.into_inner();
            expr = Expr{
                ExpType: Type::RefW,
                ident: "".to_string(),
                LHS: Some(Box::new(parse_occ(inner.next().unwrap().into_inner()))),
                RHS: Some(Box::new(parse_occ(inner.next().unwrap().into_inner()))),
                Pats: None,
                Occs: None,
            };
        }

        Rule::RefR => {
            let mut inner = rule.into_inner();
            expr = Expr{
                ExpType: Type::RefR,
                ident: "".to_string(),
                LHS: Some(Box::new(parse_occ(inner.next().unwrap().into_inner()))),
                RHS: None,
                Pats: None,
                Occs: None,
            };
        }

        _=>{
        expr = Expr{
            ExpType: Type::NONE,
            ident: "".to_string(),
            LHS: None,
            RHS: None,
            Pats: None,
            Occs: None,
        };
        }
    }

    return expr;
}

pub fn parse_occs(mut pair: Pair<Rule>) -> Vec<Box<Occ>>
{
    let res = occs(pair,Vec::new());
    return res;
}

fn occs(mut pair: Pair<Rule>, mut vec: Vec<Box<Occ>>) -> Vec<Box<Occ>>
{
    let mut inner = pair.into_inner();
    if (inner.clone().count() == 1)
    {
        vec.push(Box::new(parse_occ(inner.next().unwrap().into_inner())));
        return vec;
    }
    else
    {
        vec.push(Box::new(parse_occ(inner.next().unwrap().into_inner())));
        return occs(inner.next().unwrap(),vec);
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

    if (count > 1)
    {
        return pats(inner.next().unwrap(), vec);
    }
    return vec;
}

fn conv_pat(pat: String) -> Pat
{
    if (pat.parse::<usize>().is_ok()) { return Pat::Const(pat.parse::<usize>().unwrap()); }
    else if (pat == "_") { return Pat::Wild; }
    else { return Pat::Var(pat); }
}
