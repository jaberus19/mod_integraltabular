

//construir en base a los casos ∫x^n.sen(ax) dx, x^n.cos(ax) dx y ∫x^n e^ax dx
// Corrección de imports
use crate::modulo_integral::calculo::{derivacion, integracion,clasificar_expresion,clasificar_expresion_b};
use crate::modulo_integral::tipos::TabularIntegral;

    impl TabularIntegral{

        pub fn imprimir_tabla(&self) {
            println!("🧮 Tabla de Integración por Partes:\n");
            println!("{:<20} | {:<20}", "Derivadas de u", "Integrales de dv");
            println!("{:-<43}", "");

            for (u, dv) in self.derivadas.iter().zip(self.integrales.iter()) {
                println!("{:<20} | {:<20}", u, dv);
            }
        }

        pub fn generar_tabla(&mut self)-> &mut Self{
            
            let mut dv_actual = self.dv.clone();
            let mut u_actual = self.u.clone();

            self.derivadas.push(u_actual.clone());
            self.integrales.push(dv_actual.clone());

            let mut iter_table = 0;
            const MAX_ITERS:usize = 10;

            loop {
                
                let tipo_integral = clasificar_expresion_b(&dv_actual);
                dv_actual = integracion(&dv_actual, tipo_integral);

                let tipo_expresion_u = clasificar_expresion(&u_actual);
                u_actual = derivacion(&u_actual, tipo_expresion_u);

                self.integrales.push(dv_actual.clone()); 
                self.derivadas.push(u_actual.clone());

                if u_actual=="0" {
                    println!("DEBUG: Derivada de 'u' llegó a 0. Deteniendo tabla.");
                    break;
                    
                }
                iter_table += 1;
                if iter_table>= MAX_ITERS {
                     eprintln!("Advertencia: Se alcanzó el límite máximo de iteraciones para la tabla de integración por partes. 
                        La derivación de 'u' no llegó a 0 en este límite o hay un problema cíclico.");
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

