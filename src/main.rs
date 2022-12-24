fn main() {
    // passando valores para funções por referencia! 

    let mut name = "John";
    // passando por referencia o end mutavel da variavel 'name'
    say_hello(&mut name);
    println!("{}", name);

}

fn say_hello(name:&mut &str) {
    // deferencia a variavel name 
    *name = "Alex";
    println!("Hello {}",name);
}