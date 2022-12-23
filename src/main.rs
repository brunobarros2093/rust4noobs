use std::io;

fn main() {
    // mutable ------------- cria uma string
    let mut input = String::new();
    println!("added something: ");

    // pattern matching
    match io::stdin().read_line(&mut input) {
        // a linha do "match" pode retornar um OK ou um Error
        Ok(_) => {
            println!("said: {}", input);
        },
        Err(e) => {
            println!("error {}", e);
        }
    }
}
