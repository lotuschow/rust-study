pub mod print;

pub use crate::print::print2;
fn main() {
    print2::print_a_to_z();
    println!("------------");
    print::print_a_to_z();  
}