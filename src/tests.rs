use crate::occParser;
use crate::exprParser;
use crate::convExprToOcc;
use crate::evaluator;

    //let ReadFile = fs::read_to_string("test.df").expect("cannot read file");
    //let ReadFile = "(100^1)".to_string();
    //let ReadFile = "(x^1)".to_string();
    //let ReadFile = "((func x.(x^1)^2) (5^5)^6)".to_string();
    //let ReadFile = "(let ex (10^1) (ex^2)^3)".to_string();
    //let ReadFile = "(let x (let y (1^1) (2^2)^3) (x^4)^5)".to_string();
    //let ReadFile = "(let x (func y.(x^1)^2) ((x^3) ((x^4) (5^5)^6)^7)^8)".to_string();
    //let ReadFile = "(let x (func x.(PLUS (x^1) (1^2)^3)^4) ((x^5) ((x^6) (10^7)^8)^9)^10)".to_string();
    //let ReadFile = "(PLUS (1^1) (2^2)^3)".to_string();
    //let ReadFile = "(MINUS (5^1) (3^2)^3)".to_string();
    //let ReadFile = "(TIMES (5^1) (2^2)^3)".to_string();
    //let ReadFile = "(case (3^1) (1,2,3)((10^2),(20^3),(30^4))^5)".to_string();
    //let ReadFile = "(let x (ref (1^1)^2) (!(x^2)^3)^4)".to_string();
    //let ReadFile = "(let x (ref (1^1)^2) ((x^2):=(10^3)^4)^5)".to_string();
    
    //let ReadFile = "(letrec x (func y.(x^1)^2) (x^2)^3)".to_string();
    //let ReadFile = "(letrec x (func y.(PLUS (y^1) (1^2)^3)^4) ((x^3) (10^4)^5)^6)".to_string();
    //let ReadFile = "(letrec p (func y.(case (y^1) (p)((10^2))^5)^4) ((p^3) (10^4)^5)^6)".to_string();
    //let ReadFile = "((func x.(letrec x (func y.(case (y^1) (x)((10^2))^5)^4) ((x^3) (10^4)^5)^6)^2) (1^1)^1)".to_string();
    //let ReadFile = "(letrec p (func y.(case (y^1) (0,_)((10^2),(20^2))^5)^4) ((p^3) (0^4)^5)^6)".to_string();
    //let ReadFile = "(func y.(letrec p (func y.(case (y^1) (0,_)((10^2),(20^2))^5)^4) ((p^3) (0^4)^5)^6)^8)".to_string();
    //let ReadFile = "((func z.(letrec p (func y.(case (y^1) (0,_)((1^2),(TIMES (y^1) (2^2)^3))^5)^4) ((p^3) (5^4)^5)^6)^8) (1^10)^11)".to_string();

    //let ReadFile = "(100)".to_string();
    //let ReadFile = "((func x.(x)) (10))".to_string();
    //let ReadFile = "(PLUS (5) (5))".to_string();
    //let ReadFile = "(letrec x (func y.(y)) ((x) (5)))".to_string();
    //let ReadFile = "(let x (ref (100)) (let z ((x):=(5)) (!(x))))".to_string();

fn factorial(num: usize) -> usize
{
    match num
    {
        0 => { return 1; }
        _ => { return num * (factorial(num - 1)); }
    }
}

#[test]
fn fac()
{
    // Factorial occurrence, which uses functions, applications, recursive bindings, and functional
    // applications

    let fac_number: usize = 4;
    let res: usize = factorial(fac_number);

    let string = "((func z.(letrec fac (func y.(case (y^1) (0,_)((1^2),(TIMES (y^3) ((fac^4) (MINUS (y^5) (1^6)^7)^8)^9))^10)^11) ((fac^12) (z^13)^14)^15)^16) (".to_string() + &fac_number.to_string() + "^17)^18)"; // Factorial
    let occ = occParser::Parse_Expr(string);
    let val: evaluator::Val = evaluator::intepret(occ);
    
    assert!(match val { evaluator::Val::Const(x) => { x == res } _ => {false} });

    let string = "((func z.(letrec fac (func y.(case (y) (0,_)((1),(TIMES (y) ((fac) (MINUS (y) (1))))))) ((fac) (z)))) (".to_string() + &fac_number.to_string() + "))"; // Factorial
    let expression = exprParser::parser(string);
    let occConv = convExprToOcc::convert(expression);
    let valConv: evaluator::Val = evaluator::intepret(occConv);

    let res2: usize;
    match val { evaluator::Val::Const(x) => { res2 = x; } _ => { unreachable!() } };
    assert!(match valConv { evaluator::Val::Const(x) => { x == res && x == res2 } _ => {false} });
}

#[test]
fn references()
{
    let res: usize = 10;

    // Simple reference with read.
    let mut string = "(!(ref (10^1)^2)^3)".to_string(); // Factorial
    let mut occ = occParser::Parse_Expr(string);
    let mut val: evaluator::Val = evaluator::intepret(occ);
    assert!(match val { evaluator::Val::Const(x) => { x == res } _ => {false} });

    // Reference with aliasing before read.
    string = "(let x (ref (10^1)^2) (!(x^3)^4)^5)".to_string(); // Factorial
    occ = occParser::Parse_Expr(string);
    val = evaluator::intepret(occ);
    assert!(match val { evaluator::Val::Const(x) => { x == res } _ => {false} });

    // Reference with aliasing and write before read.
    string = "(let x (ref (100^1)^2) (let z ((x^3):=(10^4)^5) (!(x^7)^8)^9)^10)".to_string(); // Factorial
    occ = occParser::Parse_Expr(string);
    val = evaluator::intepret(occ);
    assert!(match val { evaluator::Val::Const(x) => { x == res } _ => {false} });
}
