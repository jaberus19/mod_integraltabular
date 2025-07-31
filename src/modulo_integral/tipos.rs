#[derive(Debug, PartialEq)]
pub enum TipoExpresionDerivada {
  PotenciaPsitiva,//x^n
  PotenciaNegativa,//x^-n
  Lineal,//Cx
  Constante,//C
  Cero,//0
  Desconocida,

    
}

#[derive(Debug, PartialEq)]
pub enum TipoFuncionIntegral {
    Seno,
    Coseno,
    Exponencial,
    Lineal,
    Cero,
    DesconocidaIntegral,
}
#[derive(Debug)]
 pub struct TabularIntegral{
       pub u: String,
       pub dv: String,
       pub derivadas: Vec<String>,  
       pub integrales: Vec<String>,
       pub resultado_integral_final: String,
    }

    