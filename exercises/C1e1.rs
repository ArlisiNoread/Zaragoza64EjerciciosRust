/*
    C1-E1: La medida del tiempo.

    ¿Te has preguntado alguna vez cuántos segundos han pasado desde que naciste?,
    ¿desde que aprendiste a contar?,
    ¿desde que entrastea la universidad?, o bien,
    ¿te has preguntado cuánto tiempo son un millón de segundos?
    Como todo el mundo sabe, un minuto tiene 60 segundos, una hora tiene  24 horas y un año tiene 365 días.
    Bueno, eso no siempre es cierto, pero lo consideraremos así para este ejercicio.

    Dada una cantidad de segundos s, deberás determinar cuántos años a, días d, horas h, y minutos m completos representan, además de cuántos segundos q quedan.
    
    Por ejemplo, si s = 40 000 000, entonces a = 1, d = 97, h = 23, m = 6, q = 40.
    
    @Especificación.

    La entrada consiste de un número entero s que tendrá un valor entre 0 y 2 000 000 000. 
    La salida consiste de cinco números enteros, a, d, h, m, q, separados por espacios.

    @Ejemplo.

    Entrada  | Salida
    40000000 | 1 97 23 6 40
*/

use crate::exercises::Similarity::Similarity;
use regex::Regex;
use std::str::FromStr;


pub struct C1e1;

impl Similarity for C1e1 {
    fn name(&self) -> &str{
        "La medida del tiempo."
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
    // s número de segundos
    let mut s = match i32::from_str(args) {
        Ok(s) => {
            if s > 2000000000 || s < 0 {
                return String::from("Valor debe ser menor o igual a 2000000000");
            }
            s
        },
        Err(_) => {
            return String::from("Valor debe ser menor o igual a 2000000000");
        }
    };

    //años, dias, horas, minutos, segundos
    let mut m = s/60;
    let mut h = m/60;
    let mut d = h/24;
    let a = d/365;

    d = d - (a) * 365;
    h = h - (d + a * 365) * 24;
    m = m - (h + (d + a * 365) * 24) * 60;  
    s = s - (m + (h + (d + a * 365) * 24) * 60) * 60;
    format!("{} {} {} {} {}", a, d, h, m, s)
}

