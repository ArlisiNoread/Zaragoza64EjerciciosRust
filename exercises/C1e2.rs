/*
    C1-E2: Unidades británicas de longitud.

    Con toda seguridad te has enfrentado a las unidades británicas de longitud.
    Lo que no todo el mundo sabe es cómo se relacionan unas con las otras.

    Una milla son 8 furlongs.
    Un furlong son 220 yardas.
    Una yarda son 3 pies.
    Un pie son 12 pulgadas.
    Una pulgada son... bueno, mejor lo dejamos ahí (como verás, los británicos se complicaron demasiado la vida, 
    y, no conformes, inventaron otras unidades de longitud extrañas como el rod que mide 5.5 yardas y 
    el fathom que mide 6 pies).
    
    dada una cierta cantidad de pulgadas u, deberás determinar cuántas 
    millas m, 
    furlongs f, 
    yardas y, 
    pies p, y
    pulgadas q completas representan.
    
    Por ejemplo, si se tiene que u = 40000000, entonces m = 631, f = 2, y = 111, p = 0, q = 4.

    @Especificación.

    La entrada consiste de un número entero u que tendrá un valor entre 0 y 2 000 000 000.
    La salida consiste de cinco número enteros, m, f, y, p, q, separados por espacios.

    @Ejemplo.

    Entrada  | Salida
    40000000 | 631 2 111 0 4
*/

use crate::exercises::Similarity::Similarity;
use regex::Regex;
use std::str::FromStr;


pub struct C1e2;

impl Similarity for C1e2 {
    fn name(&self) -> &str{
        "Unidades británicas de longitud."
    }

    fn start(&self, args: &str) -> String {
        solucion(args)
    }
}

fn solucion (args: &str) -> String {
    let re = Regex::new("^[0-9]*$").unwrap();
    if re.captures(args).is_none(){
        return String::from("Valor debe ser un número natural.");
    }
    // u cantidad de pulgadas
    let mut u = match i32::from_str(args) {
        Ok(u) => {
            if u > 2000000000 || u < 0 {
                return String::from("Valor debe ser menor o igual a 2000000000");
            }
            u
        },
        Err(_) => {
            return String::from("Valor debe ser menor o igual a 2000000000");
        }
    };

    //millas, furlongs, yardas, pies
    let mut p = u/12;
    let mut y = p/3;
    let mut f = y/220;
    let m = f/8;

    f = f - (m) * 8;
    y = y - (f + m * 8) * 220;
    p = p - (y + (f + m * 8) * 220) * 3;  
    u = u - (p + (y + (f + m * 8) * 220) * 3) * 12;
    format!("{} {} {} {} {}", m, f, y, p, u)
}

