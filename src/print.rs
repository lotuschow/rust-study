pub mod print2;

pub fn print_a_to_z() {
    for char in ('Z'..='a').rev() {
        println!{"{char}"}
    }
}