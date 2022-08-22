fn main() {
    //las constantes al igual que las variables son inmutables
    //pero las constantes no pueden cambiar a mutables.
    //el valor que se asigna debe de ser una expresion constante, asignacion de un valor fijo
    //o de una expresion que pueda ser resuelta en tiempo de compilacion
    //no se pueden asignar expresiones que son resueltas en tiempo de ejecucion.
    //siempre deben de declarar el tipo explicitamente.
    const SECONDS_IN_A_DAY: u32 = 60 * 60 * 24;
    //Las variables por defecto son inmutables 
    //lo que significa que una vez que se asocia un valor, no se puede cambiar
    //la inmutabilidad garantiza que una vez que se ha asignado un valor, este no sea cambiado posteriormente
    //brinda seguridad y evita estar verificando el valor de dicha variable.
    //pero cuando se requiera que una variable sea mutable, se indica con mut antes del nombre.
    let mut x = 5;
    println!("El valor de x es {x}");
    x=6; 
    println!("El valor de x es {x}");
    println!("Segundos en un dia: {SECONDS_IN_A_DAY}");

    //Rust permite shadowing en las variables, lo cual permite
    //no solo asignar nuevos valores a una variable como en el caso de mut.
    //es una nueva asignacion por completo, por lo que el tipo y el valor
    //no tienen que tener relacion con el valor previo.
    //estas son dependientes al scope.

    let y = 8;
    //El programa toma la seguiente instancia como la unica y la primera (y=8) que como unused
    let y = 9;

    {
        //hace shadow a la variable, pero solo en el contexto que es dentro de las {}
        //esta es una instancia totalmente diferente, ya que fuera de l contexto y tiene otro valor
        //implica un nuevo espacio reservado en memoria.
        let y = 10;
        println!("El valor de y en los curly-braces es {y}");
    }

    println!("El valor de y fuera de los curly-braces es {y}");
}
