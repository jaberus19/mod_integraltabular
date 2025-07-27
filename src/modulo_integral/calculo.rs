use regex::Regex;
use once_cell::sync::Lazy; 
use crate::modulo_integral::tipos::{TipoExpresionDerivada, TipoFuncionIntegral};

//varibles globales para expresiones regulares de derivacion
static  RE_POTENCIA:Lazy<Regex> = Lazy::new(|| {Regex::new(r"^(-?\d*\.?\d*)?([a-zA-Z])\^([2-9]|\d{2,})$").unwrap()});
static  RE_POTENCIA_NEGATIVA: Lazy<Regex> = Lazy::new(|| {Regex::new(r"^(\w)\^(\-\d+)$").unwrap()});
static  RE_LINEAL:Lazy<Regex> = Lazy::new(|| {Regex::new(r"^(-?\d*\.?\d*)?([a-zA-Z])$").unwrap()});
static  RE_CONSTANTE:Lazy<Regex> = Lazy::new(|| {Regex::new(r"^(-?\d+)$").unwrap()});
static  RE_CERO:Lazy<Regex> = Lazy::new(|| {Regex::new(r"^0$").unwrap()});

//variables globales para expresiones regulares de integracion
static RE_SENO:Lazy<Regex> =  Lazy::new(|| {Regex::new(r"^(-?(?:\d+(?:/\d+)?|\d*\.?\d*))\s*sen\((-?(?:\d+(?:/\d+)?|\d*\.?\d*))x\)$").unwrap()});
static RE_COSENO:Lazy<Regex> = Lazy::new(|| {Regex::new(r"^(-?(?:\d+(?:/\d+)?|\d*\.?\d*))\s*cos\((-?(?:\d+(?:/\d+)?|\d*\.?\d*))x\)$").unwrap()});
static RE_EXPONENCIAL:Lazy<Regex> = Lazy::new(|| {Regex::new(r"^(-?(?:\d+(?:/\d+)?|\d*\.?\d*))?\s*e\^(-?(?:\d*\.?\d*)?)((?:[a-zA-Z]))$").unwrap()});
static RE_LINEAL_INTEGRAL:Lazy<Regex> = Lazy::new(|| {Regex::new(r"^(-?\d*\.?\d*)?([a-zA-Z])$").unwrap()});
static RE_CERO_INTEGRAL: Lazy<Regex> = Lazy::new(|| { Regex::new(r"^0$").unwrap() });

//analiza la expresion que va a ser derivada y retorna la etiqueta enum para realizar los calulos segun el tipo
pub fn clasificar_expresion(expr_str: &str) -> TipoExpresionDerivada {
    if RE_POTENCIA.is_match(expr_str) {
        TipoExpresionDerivada::PotenciaPsitiva
    } else if RE_POTENCIA_NEGATIVA.is_match(expr_str) {
      TipoExpresionDerivada::PotenciaNegativa
    } else if RE_LINEAL.is_match(expr_str) {
        // Podrías añadir lógica aquí si 'x' y 'y' son variables válidas
        // para tu sistema, o si quieres ser más estricto.
        TipoExpresionDerivada::Lineal
    } else if RE_CONSTANTE.is_match(expr_str) {
        TipoExpresionDerivada::Constante
    } else if RE_CERO.is_match(expr_str) {
        TipoExpresionDerivada::Cero
    } else {
        TipoExpresionDerivada::Desconocida
    }
}

pub fn clasificar_expresion_b(expr_str_b: &str)-> TipoFuncionIntegral{
  if RE_SENO.is_match(expr_str_b) {
    TipoFuncionIntegral::Seno
  }else if RE_COSENO.is_match(expr_str_b) {
    TipoFuncionIntegral::Coseno
  }else if RE_EXPONENCIAL.is_match(expr_str_b) {
    TipoFuncionIntegral::Exponencial
  }else if RE_LINEAL_INTEGRAL.is_match(expr_str_b) {
      TipoFuncionIntegral::Lineal
  }else if RE_CERO_INTEGRAL.is_match(expr_str_b) {
      TipoFuncionIntegral::Cero
  }else {
      TipoFuncionIntegral::DesconocidaIntegral
  }

}

//x^2*cos(3x)
pub fn derivacion(expr_str: &str,tipo_expr:TipoExpresionDerivada ) -> String { 
  
  match tipo_expr {
    TipoExpresionDerivada::PotenciaPsitiva =>{
      
      if let Some(caps) = RE_POTENCIA.captures(expr_str) {
        let coeficiente_str = &caps[1];
        let var = &caps[2];
        let exponente_str = &caps[3];

        let coeficiente = if coeficiente_str.is_empty() {
            1.0
        } else if coeficiente_str.starts_with("-") {
            -1.0
        }else {
          coeficiente_str.parse().unwrap_or_else(|_| {
            eprintln!("Error: Coeficiente no válido en derivación: {}", coeficiente_str);
            0.0 // Devuelve 0 o maneja el error de otra forma
          })
        };


        if let Ok(exponete) = exponente_str.parse::<i32>()  {
          if exponete > 2 {
            let nuevo_coef = coeficiente*exponete as f64;
            return format!("{}{}^{}",nuevo_coef,var,exponete-1);
          }else if exponete == 2 {
              let nuevo_coef = coeficiente*exponete as f64;
            return format!("{}{}",nuevo_coef,var);
          } else if exponete == 1 {
            return format!("{}",coeficiente);   

          } else {
              return   String::from("0");
          }
            
        }else {
             eprintln!("Error: Exponente no válido: {}", exponente_str);
             return String::from("Error: Exponente inválido");
        }
          
      }else {
          eprintln!("Error: Expresión de potencia no coincide con el patrón: {}", expr_str);
          return String::from("Error: Patrón de potencia no coincide");
      }

    },

    TipoExpresionDerivada::PotenciaNegativa =>{
      if let Some(caps) = RE_POTENCIA_NEGATIVA.captures(expr_str) {
          let var = &caps[1];
          let exponentenegativo_str = &caps[2];

          if let Ok(exponentenegativo) = exponentenegativo_str.parse::<i32>() {
              return format!("{}{}^{}",exponentenegativo,var,exponentenegativo-1)
          }else {
              eprintln!("Error interno: No se pudo parsear el exponente negativo '{}'.", exponentenegativo_str);
              return String::from("Error: Problema con exponente negativo");
          }
      }else {
          eprintln!("Error: Expresión de potencia negativa no coincide con el patrón esperado: {}", expr_str);
          return String::from("Error: Patrón de potencia negativa no coincide");
      }
    },

    TipoExpresionDerivada::Lineal =>{
      
      if let Some(caps) = RE_LINEAL.captures(expr_str) {
        let constante_str = &caps[1];
        if constante_str.is_empty() {
            if expr_str.starts_with('-') {
                return String::from("-1");
            }else {
                return String::from("1");
            }
        }else {
            let constante = constante_str.parse::<f64>().unwrap_or_else(|e| {
                eprintln!("Error de parseo de coeficiente lineal: {}. Expresión: {}", e, expr_str);
                0.0 // Devuelve 0.0 si el parseo falla para evitar el pánico.
            });
            return format!("{}",constante);
        }
          
      }else {
          eprintln!("Error: Expresión lineal no coincide con el patrón: {}",expr_str);
          return String::from("Error: Patrón lineal no coincide");
      }
    },
    TipoExpresionDerivada::Constante =>{
      if let Some(caps) = RE_CONSTANTE.captures(expr_str) {
        let constante_str = &caps[1];
        if let Ok(_constante_val) =  constante_str.parse::<i32>() {
            return format!("{}",_constante_val*0);
        }else {
            eprintln!("Error: Valor de constante no parseable: {}", constante_str);
            return String::from("Error: Constante no válida");
        } 
        
        
          
      }else {
        eprintln!("Error: Expresión constante no coincide con el patrón: {}", expr_str);
        return String::from("Error: Patrón constante no coincide");
      }
    },
    TipoExpresionDerivada::Cero =>{
      if RE_CERO.is_match(expr_str) {
        // Derivative of 0 is 0
        return String::from("0");
      } else {
          eprintln!("Error: Expresión cero no coincide con el patrón: {}", expr_str);
          return String::from("Error: Patrón cero no coincide");
        }
    },
    TipoExpresionDerivada::Desconocida => {
      eprintln!("Error: Expresión desconocida: {}", expr_str);
      return String::from("Error: no de puede derivar.");
    } 
      
  }


  
  
}


pub fn integracion(expr_str_b: &str,tipo_expr_b:TipoFuncionIntegral) -> String {
    
    match tipo_expr_b {
        TipoFuncionIntegral::Coseno =>{
          if let Some(caps) = RE_COSENO.captures(expr_str_b) {
            let coeficiente_a_str = &caps[1];
            let coeficiente_b_str = &caps[2];
            
            let coeficiente_a:f64 = if coeficiente_a_str.is_empty() {
              1.0
            } 
            else{
               match coeficiente_a_str.parse::<f64>() {
                 Ok(val) => val,
                 Err(_) => {
                    // Si falla el parseo, devolvemos un String de error.
                    // Esto evita el 'return Err' que requiere un Result en la firma.
                    eprintln!("Error: No se pudo parsear el coeficiente A: '{}'", coeficiente_a_str);
                    return format!("Error: No se pudo parsear el coeficiente A: '{}'", coeficiente_a_str);
                  }
                }
              
            };
            let coeficiente_b: f64 = match coeficiente_b_str.parse::<f64>() {
              Ok(val) => val,
              Err(_) => {
                  // Si falla el parseo, devolvemos un String de error.
                  eprintln!("Error: No se pudo parsear el coeficiente B: '{}'", coeficiente_b_str);
                  return format!("Error: No se pudo parsear el coeficiente B: '{}'", coeficiente_b_str);
              }
            };
            if coeficiente_b == 0.0 {
              eprintln!("Error: El coeficiente de la variable (B) no puede ser cero para esta integral.");
              return "Error: Coeficiente B no puede ser cero para esta integral.".to_string();
            }
        
            let coeficiente_seno = coeficiente_a/coeficiente_b;
            return format!("{}sen({}x)",coeficiente_seno,coeficiente_b);

          }else {
              eprintln!("Error: La expresión no coincide con el patrón de coseno.");
              return "Error: La expresión no coincide con el patrón de coseno.".to_string();
          }
        },
        TipoFuncionIntegral::Seno =>{
          if let Some(caps) = RE_SENO.captures(expr_str_b) {
              let coeficiente_a_str = &caps[1];
              let coeficiente_b_str = &caps[2];

              let coeficiente_a:f64 = if coeficiente_a_str.is_empty() {
                1.0
              }else {
                match coeficiente_a_str.parse::<f64>() {
                    Ok(val) => val,
                    Err(_) => { 
                      eprintln!("Error: No se pudo parsear el coeficiente A: '{}'", coeficiente_a_str);
                      return format!("Error: No se pudo parsear el coeficiente A: '{}'", coeficiente_a_str);
                    }
                }
              };
              let coeficiente_b = match coeficiente_b_str.parse::<f64>() {
                  Ok(val) => val,
                  Err(_) => { 
                    eprintln!("Error: No se pudo parsear el coeficiente B: '{}'", coeficiente_b_str);
                    return format!("Error: No se pudo parsear el coeficiente B: '{}'", coeficiente_b_str);
                  }
              };
              if coeficiente_b == 0.0 {
                eprintln!("Error: El coeficiente de la variable (B) no puede ser cero para esta integral.");
                return "Error: Coeficiente B no puede ser cero para esta integral.".to_string();
              }
              let coeficiente_coseno = coeficiente_a/coeficiente_b;
              return format!("{}cos({}x)",-coeficiente_coseno,coeficiente_b);
          }else {
              eprintln!("Error: La expresión no coincide con el patrón de sen.");
              return "Error: La expresión no coincide con el patrón de seno.".to_string();
          }
        },
        TipoFuncionIntegral::Exponencial =>{
          if let Some(caps) = RE_EXPONENCIAL.captures(expr_str_b) {
            let principal_coef_str = caps.get(1).map_or("", |m| m.as_str());
            let principal_coef = if principal_coef_str.is_empty() {
                1.0
            }else {
              match principal_coef_str.parse::<f64>() {
                  Ok(val) => val,
                  Err(_) => {
                    eprintln!("Error: Coeficiente principal exponencial no parseable: '{}'", principal_coef_str);
                    return format!("Error: Coeficiente principal exponencial no válido: '{}'", principal_coef_str);
                  }
              }
            };
            
            let exponente_a_str = caps.get(2).map_or("", |m| m.as_str());
            let exponente_val_str = &caps[3];

            let exponente_a: f64 = if exponente_a_str.is_empty() {
                1.0
            }else if exponente_a_str=="-"{
                -1.0
            }else {
              match exponente_a_str.parse::<f64>() {
                  Ok(exponente_a) => exponente_a,
                  Err(_) => {
                    eprintln!("Error: No se pudo parsear el exponente A: '{}'", exponente_a_str);
                      return format!("Error: No se pudo parsear el exponente A: '{}'", exponente_a_str);
                  }
              }
            };
            if exponente_a == 0.0 {
               return format!("{:.4}{}",principal_coef,exponente_val_str);
            }else {
              let expo_new:f64 =principal_coef/exponente_a;
              return format!("{:.4}e^{:.4}{}",expo_new,exponente_a,exponente_val_str);
              
            }

          }else {
            eprintln!("Error: Expresión exponencial no coincide con el patrón después de la clasificación: {}", expr_str_b);
            return "Error: Patrón exponencial no coincide para integración".to_string();
          }
          
        },
        TipoFuncionIntegral::Lineal => {
          if let Some(caps) = RE_LINEAL_INTEGRAL.captures(expr_str_b) {
               let coef_str = caps.get(1).map_or("", |m| m.as_str());
              let var_str = &caps[2];
              let coef = if coef_str.is_empty() {
                  1.0
              }else {
                match coef_str.parse::<f64>() {
                    Ok(val) => val,
                    Err(_) => {
                      eprintln!("Error: Coeficiente lineal no parseable: '{}'", coef_str);
                      return String::from("Error: Coeficiente lineal no válido para integración");
                    },
                }
              };
              let new_coef = coef/2.0;
              return format!("{:.4}{}^2",new_coef,var_str);

          }else {
              eprintln!("Error: Expresión lineal no coincide con el patrón después de la clasificación: {}", expr_str_b);
              String::from("Error: Patrón lineal no coincide para integración")
          }
        },
        TipoFuncionIntegral::Cero => {
          return String::from("0");
        },
        TipoFuncionIntegral::DesconocidaIntegral =>{
          eprintln!("Error: Expresión desconocida: {}", expr_str_b);
          return format!("Error: Expresión desconocida para integrar: {}", expr_str_b);
        }
    }
    
  
} 

