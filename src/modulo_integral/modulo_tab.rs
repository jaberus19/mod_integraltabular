//construir en base a los casos ∫x^n.sen(ax) dx, x^n.cos(ax) dx y ∫x^n e^ax dx
use symrs::simbol::*;
use symrs::parse::parse_expr;
use symrs::diff::differentiate;
use symrs::integrate::integrate;

           

    pub struct TabularIntegral{
       pub u: String,
       pub dv: String,
       pub derivadas: Vec<String>,  
       pub integrales: Vec<String>,
    }

    #[derive(Debug)]
   pub enum CasoTabular {
        PolinomioSeno,
        PolinomioCoseno,
        PolinomioExponencial,
    }

    impl TabularIntegral{
        pub fn GenerarTabla(&mut self)-> &mut self{
            
            let mut dv_actual = self.dv.clone();
            let mut dv_anterior = self.dv.clone();
            
            while dv_actual != "0" && dv_actual !=dv_anterior{
                self.integrales.push(dv_actual.clone());
                dv_anterior=dv_actual.clone();
                
                if let Ok(expr) = parse_expr(&dv_actual) {
                    dv_actual = format!("{}", integrate(&expr, "x"));
                } else {
                    break;
                }
                 
            }

            let mut u_actual = self.u.clone();
            let mut u_anterior = self.u.clone();

            while u_actual != "0" && u_actual != u_anterior {
                self.derivadas.push(u_actual.clone());
                u_anterior = u_actual.clone();
                
                if let Ok(expr) = parse_expr(&u_actual) {
                    u_actual = format!("{}", differentiate(&expr, "x"));
                } else {
                    break;
                }
            }

            let cantidad_u = self.derivadas.len();
            let cantidad_dv = self.integrales.len();
            let min_longitud = cantidad_dv.min(cantidad_u);

            self.derivadas.truncate(min_longitud);
            self.integrales.truncate(min_longitud);


            self 
        }
    }    
  


   //let signos: Vec<&str> = (0..n).map(|i| if i % 2 == 0 { "+" } else { "-" }).collect();

