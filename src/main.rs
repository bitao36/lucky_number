
use std::io;
use rand::Rng;

fn main() {

//Necesitamos que numero sea un string mutable porque va capturar una cadena de teclado    
let mut numero_str: String=String::new();
let tope: u16=100;
let numero_secreto:u16 = rand::thread_rng().gen_range(1..=tope);
let mut sw: bool=true;
/* 
FORMA 1
    while sw 
    {
        println!("Digita un numero");
        //read_line retorna un Result que es un enum que contiene dos variant
        // Ok y Err Ese enum lo obtenemos en la variable result
        let result=io::stdin().read_line(&mut numero_str);
    
        //Ahora le daremos manejo a los datos que retornó result
        
        /* 
        match result {
            Ok(nro_bytes)=>println!("Numero de bytes capturados {nro_bytes}"),
            Err(error)=>println!("Ocurrio este error {}",error)
        }
        */
        match result {
            Ok(_nro_bytes)=>print!(""),
            Err(error)=>println!("Ocurrio este error {}",error)
        }
    
            //hace shadow de la variable numero_str que es string para que sea ahora u16
            // Se comentó para poder asignarle abajo a la variable una cadena vacia
            //let numero_str: u16 = numero_str.trim().parse().expect("Por favor digita un numero!");

            let numero: u16 = numero_str.trim().parse().expect("Por favor digita un numero!");
            
            if numero_secreto>numero
            {
                println!("El numero secreto es mayor que {}",numero);
            }    
            else if numero_secreto<numero
            {
                println!("El numero secreto es menor que {}",numero);
            }
            else
            {
                println!("Lo adivinaste, felicitaciones!");
                sw=false;
            }
            //Es importante esto porque toda la entrada de teclado se va concatenando en numero
            numero_str=String::from("");

    }
*/
//FORMA 2
loop {
    println!("Digita un numero");
    //read_line retorna un Result que es un enum que contiene dos variant
    // Ok y Err Ese enum lo obtenemos en la variable result
    let result=io::stdin().read_line(&mut numero_str);

    //Ahora le daremos manejo a los datos que retornó result
    
    /* 
    match result {
        Ok(nro_bytes)=>println!("Numero de bytes capturados {nro_bytes}"),
        Err(error)=>println!("Ocurrio este error {}",error)
    }
    */
    match result {
        Ok(_nro_bytes)=>print!(""),
        Err(error)=>println!("Ocurrio este error {}",error)
    }

        //hace shadow de la variable numero_str que es string para que sea ahora u16
        // Se comentó para poder asignarle abajo a la variable una cadena vacia
        //let numero_str: u16 = numero_str.trim().parse().expect("Por favor digita un numero!");

        let numero: u16 = numero_str.trim().parse().expect("Por favor digita un numero!");
        
        if numero_secreto>numero
        {
            println!("El numero secreto es mayor que {}",numero);
        }    
        else if numero_secreto<numero
        {
            println!("El numero secreto es menor que {}",numero);
        }
        else
        {
            println!("Lo adivinaste, felicitaciones!");
            break;
        }
        //Es importante esto porque toda la entrada de teclado se va concatenando en numero
        numero_str=String::from("");

    }

}
