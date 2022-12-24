fn main() {
    // passando valores para funções por referencia! 

    let mut name = "John";
    // passando por referencia o end mutavel da variavel 'name'
   
    println!("{}", say_hello(&mut name));

}

// --------------------------retorno esperado 
fn say_hello(name:&mut &str) -> String{
    // deferencia a variavel name 
    // println!("Hello {}",name);
    // *name = "Alex";
    let hola = format!("Hola {}", name);
    return hola; 
}