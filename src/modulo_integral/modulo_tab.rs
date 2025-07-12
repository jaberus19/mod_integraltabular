//construir en base a los casos ∫x^n.sen(ax) dx, x^n.cos(ax) dx y ∫x^n e^ax dx
use super::simbolico::{derivando, integrando};


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
        pub fn GenerarTabla(&mut self,caso:CasoTabular)->i32{
            match caso{
                CasoTabular::PolinomioSeno=>1,
                CasoTabular::PolinomioCoseno=>2,
                CasoTabular::PolinomioExponencial=>3
            }
            let mut dv_actual = integrando(self.dv.clone());
            let mut dv_anterior = self.dv.clone();
            
            while dv_actual != "0" && dv_actual !=dv_anterior{
                self.integrales.push(dv_actual.clone());
                dv_anterior=dv_actual.clone();
                dv_actual=integrando(dv_actual);
                 
            }

            let mut u_actual = derivando(self.u.clone());
            let mut u_anterior = self.u.clone();

            while u_actual != "0" && u_actual != u_anterior {
                self.derivadas.push(u_actual.clone());
                u_anterior = u_actual.clone();
                u_actual = derivando(u_actual);
            }

            let cantidad_u = self.derivadas.len();
            let cantidad_dv = self.integrales.len();
            let min_longitud = cantidad_dv.min(cantidad_u);

            self.derivadas.truncate(min_longitud);
            self.integrales.truncate(min_longitud);


            0 
        }
    }    
  