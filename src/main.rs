// crate é um grupo de modulos ------ alias
use crate::archive::arch::arch_file as arc;
// external lib, adicionada no TOML - random number generator
// {multiplos, imports} - da mesma lib
use rand::{Rng, rngs};
mod archive;


fn main() {
    // passando valores para funções por referencia! 
    // let mut name = "John";
    // passando por referencia o end mutavel da variavel 'name'
    // println!("{}", say_hello(&mut name));

    // --- UTILIZANDO MODULOS 

    arc("arquivo.txt");
    let mut rng = rand::thread_rng();
    let a:i32 = rng.gen();
    println!("Random number: {}", a);
   

}

// --------------------------retorno esperado 
// fn say_hello(name:&mut &str) -> String{
//     // deferencia a variavel name 
//     // println!("Hello {}",name);
//     // *name = "Alex";
//     let hola = format!("Hola {}", name);
//     return hola; 
// }