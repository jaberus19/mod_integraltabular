use symrs::simbol::*;
use symrs::parse::parse_expr;
use symrs::diff::differentiate;

pub fn derivacion(derivada_str:String)-> expr{
    let expr = parse_expr(derivada_str).expect("Error al parsear la expresión");
    let derivada = differentiate(&expr,"x");
}
pub fn integracion(){}
