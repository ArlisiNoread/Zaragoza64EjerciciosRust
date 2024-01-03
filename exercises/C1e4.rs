/*
    C1-E4: El calendario de la cuenta larga.
    
    Los mayas usaban tres calendarios, pero en este ejercicio nos concentraremos solamente
    en el calendario choltún, o calendario de la cuenta larga.
    
    En éste, 
    un día se llama kin,
    20 kines forman un uinal,
    18 uniales forman un tun,
    20 tunes forman un katún y
    20 katunes forman baktún
    (también existen periodos de tiempo más grandes, pero no nos ocuparemos de ellos).
    
    Una fecha se especifica como el número b de baktunes, seguido del número k de katunes,
    el número t de tunes, el número u de uinales y el número d de kines ocurridos a partir
    de una fecha inicial llamada 0.0.0.0.0 que corresponde al 11 de agosto del año 3113 antes
    de nuestra era, según el calendario gregoriano.
    Si sabemos el número n de días transcurridos a partir de esa fecha, se puede calcular
    facilmente la fecha correspondiente del calendario choltún. Por ejemplo, si han transcurrido
    314159265 días a partir de la fecha inicial, entonces la fecha correspondiente es 2182.13.4.11.5, 
    es decir b = 2191, k = 13, t = 4, u = 11, d = 5.

    @Especificación.

    La entrada consiste de un número entero n que tendrá un valor entre 0 y 2000000000.
    La salida consiste de cinco número enteros, b, k, t, u, d, separados por espacios.

    @Ejemplo.

    Entrada   | Salida
    314159265 | 2181 13 4 11 5
*/

use crate::exercises::Similarity::Similarity;
use regex::Regex;
use std::str::FromStr;


pub struct C1e4;

impl Similarity for C1e4 {
    fn name(&self) -> &str{
        "El calendario de la cuenta larga."
    }

    fn start(&self, args: &str) -> String {
        solucion(args)
    }
}

fn solucion (args: &str) -> String {
    let err = "La entrada debe ser un número natural entre 0 y 2000000000.";
    let re = Regex::new(r"^\d*$").unwrap();
    if re.captures(args).is_none(){
        return String::from(err);
    }

    let mut n = match i32::from_str(args){
        Ok(x) => {
            if x > 2000000000 {return String::from(err)}
            x
        },
        Err(_)=> {return String::from(err);}
    };

    let b = n / (20 * 20 * 18 * 20);
    n -= b * (20 * 20 * 18 * 20);

    let k = n / (20 * 18 * 20);
    n -= k * (20 * 18 * 20);

    let t = n / (18 * 20);
    n -= t * (18 * 20);

    let u = n / (20);
    n -= u * (20);

    return String::from(format!("{} {} {} {} {}", b, k, t, u, n));

}

