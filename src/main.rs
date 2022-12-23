fn main() {
    // o ! significa que a func println é um MACRO
    println!("printando");
    println!("a + b = {}", 3 + 9);
    // posições
    println!("{0} has index 0 - {2} and {1}", "pos0", "pos1", "pos2");
    println!("binario {:b}, Hex: {:x}, octal: {:o}", 5, 15, 55);
}
