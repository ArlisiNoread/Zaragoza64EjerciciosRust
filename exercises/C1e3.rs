/*
    C1-E3: Unidades británicas de volumen.
    
    Con toda seguridad te has enfrentado a las unidades británicas de volumen.
    Lo que no todo el mundo sabe es cómo se relacionan unas con otras.
    
    Un galón es igual a 4 cuartas. 
    Una cuarta es igual a 2 pintas.
    Una pinta es igual a 2 tazas.
    Una taza es igual a 8 onzas.
    
    Dada una cierta cantidad g1 de galones,
    c1 de cuartas,
    p1 de pintas,
    t1 de tazas,
    o1 de onzas,

    deberás determinar cuántos galones g2,
    cuartas c2,
    pintas p2,
    tazas t2,
    onzas o2,
    completas representan. 

    Por ejemplo, si
    g1 = 31,
    c1 = 41,
    p1 = 59,
    t1 = 26,
    o1 = 57, 
    entonces
    g2 = 50,
    c2 = 2,
    p2 = 1,
    t2 = 1,
    o2 = 1.

    @Especificación.

    La entrada consiste de cinco números enteros, g1, c1, p1, t1, o1, separados por
    espacios; cada uno tendrá un valor entre 0 y 1000. La salida consiste de cinco números enteros,
    g2, c2, p2, t2, o2, separados por espacios.

    @Ejemplo.

    Entrada        | Salida
    31 41 59 26 57 | 50 2 1 1 1
*/

use crate::exercises::Similarity::Similarity;
use regex::Regex;
use std::str::FromStr;


pub struct C1e3;

impl Similarity for C1e3 {
    fn name(&self) -> &str{
        "Unidades británicas de volumen."
    }

    fn start(&self, args: &str) -> String {
        solucion(args)
    }
}

fn solucion (args: &str) -> String {
    let re = Regex::new(r"^\d*\s\d*\s\d*\s\d*\s\d*$").unwrap();
    if re.captures(args).is_none(){
        return String::from("La entrada debe tener un formato \"d d d d d\", donde d representa un número natural entre 0 y 1000");
    }

    let nums = args.split(' ');
    let vec: Vec<&str> = nums.collect();

    let rango_error = "Los valores deben ser entre 0 y 1000";
    for val in &vec {
        match i32::from_str(val){
            Ok(x) => {
                if x > 1000 {return String::from(rango_error)}
            },
            Err(_)=> return String::from(rango_error)
        }
    }

    let error_message = "Error que no debería pasar al convertir las unidades";
    let g1 = i32::from_str(vec[0]).expect(error_message);
    let c1 = i32::from_str(vec[1]).expect(error_message);
    let p1 = i32::from_str(vec[2]).expect(error_message);
    let t1 = i32::from_str(vec[3]).expect(error_message);
    let o1 = i32::from_str(vec[4]).expect(error_message);

    
    let mut o2 = o1;
    o2 += t1 * 8;
    o2 += p1 * 2 * 8;
    o2 += c1 * 2 * 2 * 8;
    o2 += g1 * 4 * 2 * 2 * 8;

    let g2 = o2 / (4 * 2 * 2 * 8);
    o2 -= g2 * (4 * 2 * 2 * 8);

    let c2 = o2 / (2 * 2 * 8);
    o2 -= c2 * (2 * 2 * 8);

    let p2 = o2 / (2 * 8);
    o2 -= p2 * (2 * 8);

    let t2 = o2 / (8);
    o2 -= t2 * (8);

    return String::from(format!("{} {} {} {} {}", g2, c2, p2, t2, o2));

}

