mod exercises;

fn main(){
    println!("Ejecución realizada \n");
    /*
    let ejercicio = "c1e1";
    let valores = "40000000";
    */
    /*
    let ejercicio = "c1e2";
    let valores = "40000000";
    */
    /*
    let ejercicio = "c1e3";
    let valores = "31 41 59 26 57";
    */
    let ejercicio = "c1e4";
    let valores = "314159265";

    let obj = exercises::factory(ejercicio);
    match obj {
        Some(x) => {
            println!("{}: {}", ejercicio.to_uppercase(), x.name());
            println!("{}", valores);
            println!("{}", x.start(valores));
        },
        _ => println!("Ejercicio no definido."),
    }

}
